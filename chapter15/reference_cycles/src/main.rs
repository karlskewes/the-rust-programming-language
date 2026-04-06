use reference_cycles::ref_cycle;
use reference_cycles::weak_ref;

fn main() {
    ref_cycle::create_ref_cycles();

    weak_ref::part_1_child_cant_access_parent();
    weak_ref::part_2_child_can_access_parent();
    weak_ref::part_3_visualizing_changes();
}
