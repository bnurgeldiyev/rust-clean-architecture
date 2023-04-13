#[cfg(test)]
mod tests {
    use crate::internal::user::entity::user::{
        UserAuthRequest, UserCreateRequest,
        verify_user_auth_request, verify_user_create_request
    };

    struct TestCase<T, A> {
        input: A,
        output: T,
    }

    #[test]
    fn verify_user_auth_request_test() {
        let test_cases = vec! {
            TestCase {
                input: UserAuthRequest {
                    username: String::from("James"),
                    password: String::from("james123"),
                },
                output: Ok(()),
            },
            TestCase {
                input: UserAuthRequest {
                    username: String::from(""),
                    password: String::from("james123"),
                },
                output: Err("invalid username".to_string()),
            },
            TestCase {
                input: UserAuthRequest {
                    username: String::from("James"),
                    password: String::from(""),
                },
                output: Err("invalid password".to_string()),
            },
        };

        for test_case in test_cases {
            let res = verify_user_auth_request(test_case.input);
            assert_eq!(res, test_case.output)
        }
    }

    #[test]
    fn verify_user_create_request_test() {
        let test_cases = vec! {
            TestCase {
                input: UserCreateRequest {
                    username: String::from("JamesHolland"),
                    password: String::from("james123"),
                    firstname: String::from("James"),
                    lastname: String::from("Holland")
                },
                output: Ok(()),
            },
            TestCase {
                input: UserCreateRequest {
                    username: String::from(""),
                    password: String::from("james123"),
                    firstname: String::from("James"),
                    lastname: String::from("Holland")
                },
                output: Err("invalid username".to_string()),
            },
            TestCase {
                input: UserCreateRequest {
                    username: String::from("JamesHolland"),
                    password: String::from(""),
                    firstname: String::from("James"),
                    lastname: String::from("Holland")
                },
                output: Err("invalid password".to_string()),
            },
            TestCase {
                input: UserCreateRequest {
                    username: String::from("JamesHolland"),
                    password: String::from("james123"),
                    firstname: String::from(""),
                    lastname: String::from("Holland")
                },
                output: Err("firstname is empty".to_string()),
            },
            TestCase {
                input: UserCreateRequest {
                    username: String::from("JamesHolland"),
                    password: String::from("james123"),
                    firstname: String::from("James"),
                    lastname: String::from("")
                },
                output: Err("lastname is empty".to_string()),
            },
        };

        for test_case in test_cases {
            let res = verify_user_create_request(test_case.input);
            assert_eq!(res, test_case.output)
        }
    }
}
