use notify::EventKind;

pub fn kind_of_event(event: &EventKind) -> String {
    if event.is_access() {
        "access".to_string()
    } else if event.is_create() {
        "create".to_string()
    } else if event.is_modify() {
        "modify".to_string()
    } else if event.is_remove() {
        "remove".to_string()
    } else if event.is_other() {
        "other".to_string()
    } else {
        "unknown".to_string()
    }
}
