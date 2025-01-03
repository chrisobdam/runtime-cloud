use std::io::Read;
use wasmcloud_component::http;
wit_bindgen::generate!({ generate_all });
// use crate::wasi::logging::logging::*;
use http::Method;
use juniper::{
    graphql_object, EmptySubscription, GraphQLEnum, GraphQLInputObject, GraphQLObject, Variables,
};

use juniper::FieldResult;

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    // Note, that the enum value will be automatically converted to the
    // `SCREAMING_SNAKE_CASE` variant, just as GraphQL conventions imply.
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: i32,
    name: String,
    // appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
struct NewHuman {
    name: String,
    // appears_in: Vec<Episode>,
    home_planet: String,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object]
#[graphql(context = Ctx)]
impl Query {
    fn favorite_episode(context: &Ctx) -> Episode {
        context.0
    }
    fn all_human() -> FieldResult<Human> {
        Ok(Human {
            id: 1,
            name: "chris".to_string(),
            home_planet: "tatooine".to_string(),
        })
    }
}

struct Mutation;

#[graphql_object]
#[graphql(
    context = Ctx,
)]
impl Mutation {
    fn action() -> FieldResult<Human> {
        // Logic to create a new human in the database
        let human = Human {
            id: 1, // Replace with actual ID
            name: "Henk".to_string(),
            home_planet: "home".to_string(),
        };

        Ok(human)
    }
}
// mutation {
//     action(id: "1de601200e7e42559952df0b37c150ad", input: $input)
//   }

type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Ctx>>;

fn gql(query: &str) -> String {
    // Create a context.
    let ctx = Ctx(Episode::NewHope);

    // Run the execution.
    let (res, _errors) = juniper::execute_sync(
        query,
        None,
        &Schema::new(Query, Mutation, EmptySubscription::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    format!("{}", res)
}

fn get_config() -> String {
    wasi::config::runtime::get("bb_runtime_cloud-gql_config")
        .expect("Unable to fetch value")
        .unwrap_or_else(|| "config value not set".to_string())
}

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        // let authorization_header = get_authorization_header(&_request);
        // let method = get_method(&_request);
        // let mut body_vec = Vec::new();

        let (_parts, mut body) = _request.into_parts();
        let mut buf = vec![];
        body.read_to_end(&mut buf)
            .expect("should have read incoming buffer");

        let body_text = String::from_utf8(buf).expect("no valid UTF8");
        let authorization_header = _parts
            .headers
            .get("Authorization")
            .map(|value| value.to_str().unwrap().to_string());
        match _parts.method {
            Method::POST => {
                let str = bettyblocks::runtime_cloud::action_runner::execute();

                return Ok(http::Response::new(format!(
                    "{} {:?} {}",
                    body_text, authorization_header, str
                )));
            }
            _ => {
                // Handle non POST request logic here
                return Ok(http::Response::new(format!(
                    "Only POST requests are allowed"
                )));
            }
        }
        // let str = format!(
        //     "Hallo, {} {} {}!",
        //     gql("query { favoriteEpisode }"),
        //     str,
        //     get_config()
        // );
        // Ok(http::Response::new(str))
    }
}

#[cfg(test)]
mod tests {
    // use wasi::http::outgoing_handler::handle;

    use super::*;

    #[test]
    fn it_works() {
        let _query = "mutation { action(id:\"henk\"){ name } }";

        let ctx = Ctx(Episode::NewHope);

        // Run the execution.
        let res = juniper::execute_sync(
            _query,
            None,
            &Schema::new(Query, Mutation, EmptySubscription::new()),
            &Variables::new(),
            &ctx,
        );

        assert!(res.is_ok());
        let (result, _err) = res.unwrap();
        assert_eq!(result.to_string(), "{\"action\": {\"name\": \"Henk\"}}");
    }

    fn doesnt_work() {
        let _query = "mutation { ction(id:\"henk\"){ name } }";

        let ctx = Ctx(Episode::NewHope);

        // Run the execution.
        let res = juniper::execute_sync(
            _query,
            None,
            &Schema::new(Query, Mutation, EmptySubscription::new()),
            &Variables::new(),
            &ctx,
        );

        assert!(res.is_err());
        // let (result, _err) = res.unwrap();
        // for error in _err {
        //     // Handle each error
        //     println!("GraphQL Error: {}", error.message);
        //     // You can further customize error handling here:
        //     // - Log the error to a file
        //     // - Return a custom error response to the client
        //     // - Trigger alerts or notifications
        // }
        // let str = format!("{}", _err[0].to_string());
        // assert_eq!(_err, ExecutionError.new());
    }
}
