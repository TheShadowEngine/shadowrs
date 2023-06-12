use bitvec::prelude::*;
use shadow_progress_counter::ProgressCounter;

pub mod compute_binning_instructions;
#[cfg(feature = "timing")]
mod timing;

pub struct Progress<'a> {
    pub kill_chip: &'a shadow_kill_chip::KillChip,
    pub handle_progress_event: &'a mut dyn FnMut(TrainProgressEvent),
}

#[derive(Clone, Debug)]
pub struct TrainOptions {
    pub binned_features_layout: BinnedFeaturesLayout,
    pub compute_losses: bool,
    pub early_stopping_options: Option<EarlyStoppingOptions>,
    pub l2_regularization_for_continuous_splits: f32,
    pub l2_regularization_for_discrete_splits: f32,
    pub learning_rate: f32,
    pub max_depth: Option<usize>,
    pub max_examples_for_computing_bin_thresholds: usize,
    pub max_leaf_nodes: usize,
    pub max_rounds: usize,
    pub max_valid_bins_for_number_features: u8,
    pub min_examples_per_node: usize,
    pub min_gain_to_split: f32,
    pub min_sum_hessians_per_node: f32,
    pub smoothing_factor_for_discrete_bin_sorting: f32,
}

impl Default for TrainOptions {
    fn default() -> TrainOptions {
        TrainOptions {
            binned_features_layout: BinnedFeaturesLayout::ColumnMajor,
            compute_losses: false,
            early_stopping_options: None,
            l2_regularization_for_continuous_splits: 0.0,
            l2_regularization_for_discrete_splits: 10.0,
            learning_rate: 0.1,
            max_depth: None,
            max_leaf_nodes: 31,
            max_rounds: 100,
            max_valid_bins_for_number_features: 255,
            min_examples_per_node: 20,
            min_gain_to_split: 0.0,
            min_sum_hessians_per_node: 1e-3,
            max_examples_for_computing_bin_thresholds: 200_000,
            smoothing_factor_for_discrete_bin_sorting: 10.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinnedFeaturesLayout {
    RowMajor,
    ColumnMajor,
}

#[derive(Clone, Debug)]
pub struct EarlyStoppingOptions {
    pub early_stopping_fraction: f32,
    pub n_rounds_without_improvement_to_stop: usize,
    pub min_decrease_in_loss_for_significant_change: f32,
}

#[derive(Clone, Debug)]
pub enum TrainProgressEvent {
    Initialize(ProgressCounter),
    InitializeDone,
    Train(ProgressCounter),
    TrainDone,
}

#[derive(Clone, Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn predict(&self, example: &[shadow_table::TableValue]) -> f32 {
        let mut node_index = 0;
        unsafe {
            loop {
                match self.nodes.get_unchecked(node_index) {
                    Node::Leaf(LeafNode { value, .. }) => return *value as f32,
                    Node::Branch(BranchNode {
                        left_child_index,
                        right_child_index,
                        split:
                            BranchSplit::Continuous(BranchSplitContinuous {
                                feature_index,
                                split_value,
                                ..
                            }),
                        ..
                    }) => {
                        node_index = if example.get_unchecked(*feature_index).as_number().unwrap()
                            <= split_value
                        {
                            *left_child_index
                        } else {
                            *right_child_index
                        };
                    }

                    Node::Branch(BranchNode {
                        left_child_index,
                        right_child_index,
                        split:
                            BranchSplit::Discrete(BranchSplitDiscrete {
                                feature_index,
                                directions,
                                ..
                            }),
                        ..
                    }) => {
                        let bin_index = if let Some(bin_index) =
                            example.get_unchecked(*feature_index).as_enum().unwrap()
                        {
                            bin_index.get()
                        } else {
                            0
                        };
                        let direction = (*directions.get(bin_index).unwrap()).into();
                        node_index = match direction {
                            SplitDirection::Left => *left_child_index,
                            SplitDirection::Right => *right_child_index,
                        };
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Branch(BranchNode),
    Leaf(LeafNode),
}

impl Node {
    pub fn as_branch(&self) -> Option<&BranchNode> {
        match self {
            Node::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    pub fn as_leaf(&self) -> Option<&LeafNode> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    pub fn examples_fraction(&self) -> f32 {
        match self {
            Node::Leaf(LeafNode {
                examples_fraction, ..
            }) => *examples_fraction,
            Node::Branch(BranchNode {
                examples_fraction, ..
            }) => *examples_fraction,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BranchNode {
    pub left_child_index: usize,
    pub right_child_index: usize,
    pub split: BranchSplit,
    pub examples_fraction: f32,
}

#[derive(Clone, Debug)]
pub enum BranchSplit {
    Continuous(BranchSplitContinuous),
    Discrete(BranchSplitDiscrete),
}

#[derive(Clone, Debug)]
pub struct BranchSplitContinuous {
    pub feature_index: usize,
    pub split_value: f32,
    pub invalid_values_direction: SplitDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitDirection {
    Left,
    Right,
}

impl From<bool> for SplitDirection {
    fn from(value: bool) -> Self {
        match value {
            false => SplitDirection::Left,
            true => SplitDirection::Right,
        }
    }
}

impl From<SplitDirection> for bool {
    fn from(value: SplitDirection) -> Self {
        match value {
            SplitDirection::Left => false,
            SplitDirection::Right => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BranchSplitDiscrete {
    pub feature_index: usize,
    pub directions: BitVec<u8, Lsb0>,
}

#[derive(Clone, Debug)]
pub struct LeafNode {
    pub value: f64,
    pub examples_fraction: f32,
}

impl BranchSplit {
    pub fn feature_index(&self) -> usize {
        match self {
            BranchSplit::Continuous(s) => s.feature_index,
            BranchSplit::Discrete(s) => s.feature_index,
        }
    }
}
