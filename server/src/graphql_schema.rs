use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
enum PermissionGroups {
    Default,
    Admin
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Account {
    id: String,
    name: String,
    date_joined: String,
}
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewAccount {
    name: String,
    date_joined: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A user that belongs to an account")]
struct User {
    id: String,
    username: String,
    permission_groups: Vec<PermissionGroups>,
    account_id: String,
}
#[derive(GraphQLInputObject)]
#[graphql(description = "A user that belongs to an account")]
struct NewUser {
    username: String,
    permission_groups: Vec<PermissionGroups>,
    account_id: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    #[graphql(description = "get an account, given an ID")]
    fn account(id: String) -> FieldResult<Account> {
        Ok(Account {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            date_joined: "123".to_owned(),
        })
    }

    #[graphql(description = "get a user, given an ID")]
    fn user(id: String) -> FieldResult<User> {
        Ok(User {
            id: "1234".to_owned(),
            username: "Luke".to_owned(),
            permission_groups: vec![PermissionGroups::Default],
            account_id: "abc".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    #[graphql(description = "create a new user")]
    fn create_user(new_user: NewUser) -> FieldResult<User> {
        Ok(User {
            id: "1234".to_owned(),
            username: new_user.username,
            permission_groups: new_user.permission_groups,
            account_id: new_user.account_id,
        })
    }
    
    #[graphql(description = "create a new account")]
    fn create_account(new_account: NewAccount) -> FieldResult<Account> {
        Ok(Account {
            id: "1234".to_owned(),
            name: new_account.name,
            date_joined: new_account.date_joined,
        })
    }

}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
