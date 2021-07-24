use buttercup_api::bts::action::logging::PrintLogActionNodeDefinition;
use buttercup_api::bts::action::subtree::ExecuteSubTreeActionNodeDefinition;
use buttercup_api::bts::composite::sequence::SequenceCompositeNodeDefinition;
use std::sync::Arc;

mod common;

#[test]
fn test_builds_subtree_node_correctly() {
    let subtree_id = 10;

    let tree_definition =
        common::one_off_root_tree(2,
                                  vec![
                                      Arc::new(
                                          ExecuteSubTreeActionNodeDefinition::new(
                                              2, subtree_id))
                                  ]);

    common::build_with_subtrees(tree_definition,
                                vec![
                                    common::one_off_root_tree_with_id(
                                        2,
                                        vec![
                                            Arc::new(
                                                PrintLogActionNodeDefinition::new(
                                                2, "I'm a subtree!".to_owned())
                                            )
                                        ],
                                        subtree_id)])
        .expect("Expected the build to succeed!");
}

#[test]
fn test_builds_multiple_subtree_nodes_correctly() {

}