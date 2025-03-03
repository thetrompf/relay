/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @generated SignedSource<<25dc040b7af5206f27961327f19696d7>>
 */

mod relay_actor_change;

use relay_actor_change::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn actor_change_invalid() {
    let input = include_str!("relay_actor_change/fixtures/actor-change.invalid.graphql");
    let expected = include_str!("relay_actor_change/fixtures/actor-change.invalid.expected");
    test_fixture(transform_fixture, "actor-change.invalid.graphql", "relay_actor_change/fixtures/actor-change.invalid.expected", input, expected);
}

#[test]
fn simple_query() {
    let input = include_str!("relay_actor_change/fixtures/simple-query.graphql");
    let expected = include_str!("relay_actor_change/fixtures/simple-query.expected");
    test_fixture(transform_fixture, "simple-query.graphql", "relay_actor_change/fixtures/simple-query.expected", input, expected);
}
