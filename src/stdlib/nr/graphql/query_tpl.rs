
pub fn make_nrql_query(a: String, q: String) -> String {
    format!(r#"
    {{
       actor {{
          account(id: {}) {{
             nrql(query: "{}") {{
                results
            }}
         }}
      }}
    }}
    "#, a, q).to_string()
}
