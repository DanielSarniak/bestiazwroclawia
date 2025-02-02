use move_gen::r#move::{Move, MoveKind};

use crate::engine::search::MATE_SCORE;

const LMP_DEPTH: usize = 6;
const LMP: [usize; LMP_DEPTH] = [999, 3, 6, 10, 15, 21];

#[must_use]
pub fn is_lmp_applicable(
    order: usize,
    depth: usize,
    pv_node: bool,
    in_check: bool,
    alpha: i32,
    mv: &Move,
) -> bool {
    depth < LMP_DEPTH
        && order > LMP[depth]
        && !pv_node
        && !in_check
        && alpha.abs() < MATE_SCORE
        && !matches!(
            mv.kind(),
            MoveKind::Capture | MoveKind::Promotion | MoveKind::PromotionCapture
        )
}
