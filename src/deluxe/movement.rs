use crate::deluxe::game_object::GameObject;

pub fn go_left(objects: &mut Vec<GameObject>) {
    for obj in objects {
        if obj.driveable && obj.x > 0 {
            obj.x -= 1;
            obj.sigil_index = 2;
        }
    }
}

pub fn go_right(objects: &mut Vec<GameObject>, width: i16) {
    for obj in objects {
        if obj.driveable && obj.x < width - 1 {
            obj.x += 1;
            obj.sigil_index = 3;
        }
    }
}

pub fn go_up(objects: &mut Vec<GameObject>) {
    for obj in objects {
        if obj.driveable && obj.y > 0 {
            obj.y -= 1;
            obj.sigil_index = 0;
        }
    }
}

pub fn go_down(objects: &mut Vec<GameObject>, height: i16) {
    for obj in objects {
        if obj.driveable && obj.y < height - 1 {
            obj.y += 1;
            obj.sigil_index = 1;
        }
    }
}
