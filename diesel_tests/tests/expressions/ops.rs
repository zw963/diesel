use schema::*;
use diesel::*;

#[test]
fn adding_literal_to_column() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![2, 3];
    let data = users.select(id + 1).load(&connection);
    assert_eq!(Ok(expected_data), data);

    let expected_data = vec![3, 4];
    let data = users.select(id + 2).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
#[cfg(not(feature="sqlite"))] // FIXME: Does SQLite provide a way to detect overflow?
fn overflow_returns_an_error_but_does_not_panic() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();
    let query_result = users.select(id + i32::max_value()).load::<i32>(&connection);
    assert!(query_result.is_err(), "Integer overflow should have returned an error");
}

#[test]
fn adding_column_to_column() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![2, 4];
    let data = users.select(id + id).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn adding_multiple_times() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![4, 5];
    let data = users.select(id + 1 + 2).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn subtracting_literal_from_column() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![0, 1];
    let data = users.select(id - 1).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn adding_then_subtracting() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![2, 3];
    let data = users.select(id + 2 - 1).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn multiplying_column() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![3, 6];
    let data = users.select(id * 3).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn dividing_column() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();

    let expected_data = vec![0, 1];
    let data = users.select(id / 2).load(&connection);
    assert_eq!(Ok(expected_data), data);
}

#[test]
fn mix_and_match_all_numeric_ops() {
    use schema::users::dsl::*;

    let connection = connection_with_sean_and_tess_in_users_table();
    connection.execute("INSERT INTO users (id, name) VALUES
        (3, 'Jim'), (4, 'Bob')").unwrap();

    let expected_data = vec![4, 6, 7, 9];
    let data = users.select(id * 3 / 2 + 4 - 1).load(&connection);
    assert_eq!(Ok(expected_data), data);
}
