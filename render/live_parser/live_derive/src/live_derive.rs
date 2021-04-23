use proc_macro::{TokenStream};

use crate::macro_lib::*;
use crate::id::*;

pub fn live_derive_impl(input: TokenStream) -> TokenStream {
    let mut parser = TokenParser::new(input);
    let mut tb = TokenBuilder::new();
    parser.eat_ident("pub");
    
    if parser.eat_ident("struct") {
        if let Some(name) = parser.eat_any_ident() {
            let generic = parser.eat_generic();
            let types = parser.eat_all_types();
            let where_clause = parser.eat_where_clause(Some("DeLive"));
            
            tb.add("impl").stream(generic.clone());
            tb.add("DeLive for").ident(&name).stream(generic).stream(where_clause);
            tb.add("{ fn de_live ( lr : & LiveRegistry , f : usize , l : usize , s : usize )");
            tb.add("-> std :: result :: Result < Self , DeLiveErr > { ");
            
            tb.add("let doc = & lr . expanded [ f ] . as_ref ( ) . unwrap ( ) ;");
            tb.add("let cn = & doc . nodes [ l ] [ s ]  ;");
            
            // forward if just an ID
            tb.add("if let LiveValue :: Id ( id ) = cn . value {");
            tb.add("if let IdType :: NodePtr { file_id , ptr } = id . to_type ( ) {");
            tb.add("return DeLive :: de_live ( lr , file_id . to_index ( ) , ptr . level , ptr . index ) ;");
            tb.add("}");
            tb.add("}");
            
            if let Some(types) = types { // we can support this!
                
                tb.add("if let LiveValue :: Call { node_start , node_count , .. } = cn . value {");
                tb.add("if node_count < ").unsuf_usize(types.len()).add("{");
                tb.add("return Err ( DeLiveErr :: arg_count ( cn . id , node_count as usize ,").unsuf_usize(types.len()).add(", f , l , s ) ) ;");
                tb.add("}");
                tb.add("let ln = l + 1 ;");
                tb.add("let ns = node_start as usize ;");
                tb.add("return std :: result :: Result :: Ok ( Self (");
                for i in 0..types.len() {
                    tb.add("DeLive :: de_live ( lr , f , ln , ns +").unsuf_usize(i).add(") ? ,");
                }
                tb.add(") ) }");
                tb.add("else {");
                tb.add("return Err ( DeLiveErr :: not_class ( cn , f , l , s ) )");
                tb.add("}");
            }
            else if let Some(fields) = parser.eat_all_struct_fields() { // if all our fields are f32's
                
                tb.add("if let LiveValue :: Class { node_start , node_count , .. } = cn . value {");
                tb.add("let ln = l + 1 ;");
                
                for field in &fields {
                    tb.add("let mut").ident(&format!("_{}", field.name)).add("= None ;");
                }
                tb.add("for i in 0 .. ( node_count as usize ) {");
                tb.add("let si = i + ( node_start as usize ) ;");
                tb.add("let n = & doc . nodes [ ln ] [ si ] ;");
                tb.add("match n . id {");
                for field in &fields {
                    // lets id it
                    let id = Id::from_str(&field.name);
                    tb.add("Id (").suf_u64(id.0).add(") =>");
                    tb.ident(&format!("_{}", field.name)).add("= Some ( DeLive :: de_live ( lr , f , ln , si ) ? ) ,");
                }
                tb.add("_ => ( )");
                tb.add("} }");
                
                tb.add("return std :: result :: Result :: Ok ( Self {");
                for field in fields {
                    tb.ident(&field.name).add(":");
                    if field.ty.into_iter().next().unwrap().to_string() == "Option" {
                        tb.add("if let Some ( t ) = ").ident(&format!("_{}", field.name));
                        tb.add("{ Some ( t ) } else { None } ,");
                    }
                    else {
                        tb.add("if let Some ( t ) =").ident(&format!("_{}", field.name));
                        tb.add("{ t } else { return Err ( DeLiveErr :: miss_prop ( cn . id ,").string(&field.name).add(", f , l , s ) ) } ,");
                    }
                }
                tb.add("} ) }");
                
                tb.add("else {");
                tb.add("return Err ( DeLiveErr :: not_class ( cn , f , l , s ) )");
                tb.add("}");
            }
            else {
                return parser.unexpected()
            }
            tb.add("} } ;");
            return tb.end();
        }
    }
    else if parser.eat_ident("enum") {
        
        if let Some(name) = parser.eat_any_ident() {
            let generic = parser.eat_generic();
            let where_clause = parser.eat_where_clause(Some("DeLive"));
            
            tb.add("impl").stream(generic.clone());
            tb.add("DeLive for").ident(&name).stream(generic).stream(where_clause);
            tb.add("{ fn de_live ( lr : & LiveRegistry , f : usize , l : usize , s : usize )");
            tb.add("-> std :: result :: Result < Self , DeLiveErr > { ");
            
            if !parser.open_brace() {
                return parser.unexpected()
            }
            let mut named = Vec::new();
            let mut bare = Vec::new();
            let mut unnamed = Vec::new();
            while !parser.eat_eot() {
                
                if let Some(variant) = parser.eat_any_ident() {
                    if let Some(types) = parser.eat_all_types() {
                        unnamed.push((variant, types));
                    }
                    else if let Some(fields) = parser.eat_all_struct_fields() { // named variant
                        named.push((variant, fields))
                    }
                    else if parser.is_punct(',') || parser.is_eot() { // bare variant
                        bare.push(variant)
                    }
                    else {
                        return parser.unexpected();
                    }
                }
                
                parser.eat_punct(',');
            }
            // alright lets write out our matcher
            tb.add("let doc = & lr . expanded [ f ] . as_ref ( ) . unwrap ( ) ;");
            tb.add("let cn = & doc . nodes [ l ] [ s ]  ;");
            
            tb.add("match cn . value {");
            
            if bare.len()>0 {
                tb.add("LiveValue :: Id ( id ) => {");
                tb.add("let orig_id = lr . find_enum_origin ( id , id ) ;");
                tb.add("match orig_id {");
                for variant in bare {
                    let id = Id::from_str(&variant);
                    tb.add("Id (").suf_u64(id.0).add(") =>");
                    tb.add("return Ok ( Self ::").ident(&variant).add(") ,");
                }
                tb.add("_ => return Err ( DeLiveErr :: enum_notfound ( orig_id , cn . id , f , l , s ) )");
                tb.add("}");
                tb.add("}");
            }
            
            if unnamed.len()>0 {
                tb.add("LiveValue :: Call { target , node_start , node_count } => {");
                tb.add("let orig_id = lr . find_enum_origin ( target , target ) ;");
                tb.add("match orig_id {");
                for (variant, types) in unnamed {
                    let id = Id::from_str(&variant);
                    tb.add("Id (").suf_u64(id.0).add(") => {");
                    // ok now we need to parse the arguments
                    
                    tb.add("if node_count < ").unsuf_usize(types.len()).add("{");
                    tb.add("return Err ( DeLiveErr :: arg_count ( cn . id , node_count as usize ,").unsuf_usize(types.len()).add(", f , l , s ) ) ;");
                    tb.add("}");
                    tb.add("let ln = l + 1 ;");
                    tb.add("let ns = node_start as usize ;");
                    tb.add("return std :: result :: Result :: Ok ( Self ::").ident(&variant).add("(");
                    for i in 0..types.len() {
                        tb.add("DeLive :: de_live ( lr , f , ln , ns +").unsuf_usize(i).add(") ? ,");
                    }
                    tb.add(") ) } ,");
                }
                tb.add("_ => return Err ( DeLiveErr :: enum_notfound ( orig_id , cn . id , f , l , s ) )");
                tb.add("}");
                tb.add("}");
            }
            
            if named.len()>0 {
                tb.add("LiveValue :: Class { class , node_start , node_count } => {");
                tb.add("let orig_id = lr . find_enum_origin ( class , class ) ;");
                tb.add("match orig_id {");
                for (variant, fields) in named {
                    let id = Id::from_str(&variant);
                    tb.add("Id (").suf_u64(id.0).add(") => {");
                    
                    tb.add("let ln = l + 1 ;");
                    
                    for field in &fields {
                        tb.add("let mut").ident(&format!("_{}", field.name)).add("= None ;");
                    }
                    tb.add("for i in 0 .. ( node_count as usize ) {");
                    tb.add("let si = i + ( node_start as usize ) ;");
                    tb.add("let n = & doc . nodes [ ln ] [ si ] ;");
                    tb.add("match n . id {");
                    for field in &fields {
                        // lets id it
                        let id = Id::from_str(&field.name);
                        tb.add("Id (").suf_u64(id.0).add(") =>");
                        tb.ident(&format!("_{}", field.name)).add("= Some ( DeLive :: de_live ( lr , f , ln , si ) ? ) ,");
                    }
                    tb.add("_ => ( )");
                    tb.add("} }");
                    
                    tb.add("return std :: result :: Result :: Ok ( Self ::").ident(&variant).add("{");
                    for field in fields {
                        tb.ident(&field.name).add(":");
                        if field.ty.into_iter().next().unwrap().to_string() == "Option" {
                            tb.add("if let Some ( t ) = ").ident(&format!("_{}", field.name));
                            tb.add("{ Some ( t ) } else { None } ,");
                        }
                        else {
                            tb.add("if let Some ( t ) =").ident(&format!("_{}", field.name));
                            tb.add("{ t } else { return Err ( DeLiveErr :: miss_prop ( cn . id ,").string(&field.name).add(", f , l , s ) ) } ,");
                        }
                    }
                    tb.add("} ) } ,");
                }
                tb.add("_ => return Err ( DeLiveErr :: enum_notfound ( orig_id , cn . id , f , l , s ) )");
                tb.add("}");
                tb.add("}");
            }
            
            tb.add("_ => ( )");
            tb.add("}");
            
            tb.add("return Err ( DeLiveErr :: enum_notfound ( Id :: empty ( ) , cn . id , f , l , s ) )");
            tb.add("} }");
            return tb.end();
        }
    }
    return parser.unexpected()
}