// Tutorial: Radar
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Press 'n' to single-step.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
//
// Join the Discord at https://discord.gg/vYyu9EhkKH for Oort discussion and
// tournament results.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s


fn getAngle(originPosition: Vec2, originVelocity: Vec2, target: Vec2,  targetVelocity: Vec2) -> Option<f64> {
    let dp = target - originPosition;
    let relative_velocity = targetVelocity - originVelocity;
    
    let a = relative_velocity.length().powi(2) - BULLET_SPEED.powi(2);
    let b = 2.0 * dp.dot(relative_velocity);
    let c = dp.length().powi(2);

    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let time_to_impact = if t1 > 0.0 { t1 } else { t2 };

        if time_to_impact > 0.0 {
            let lead_target = target + targetVelocity * time_to_impact;

            draw_line(originPosition, lead_target, 0xff00ff);

            let degrees_to_turn = angle_diff(heading(), lead_target.angle());
            return Some(degrees_to_turn);
        }
    } 
    None
}

pub struct Ship {
    contact: Option<ScanResult>,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            contact: None,
        }
    }



    pub fn tick(&mut self) {
        let mut sawContact = false;
        let mut recentContact = None;
        debug!("has contact: {}", recentContact.is_some());

        if let Some(contact) = scan() {
            sawContact = true;
            debug!("Contact detected: {}", contact.position);
            recentContact = Some(contact);
        }

        match recentContact {
            Some(enemy) => {
                if let Some(angle_to_contact) = getAngle(position(), velocity(), enemy.position, enemy.velocity) {
                    debug!("ANGLE_TO_CONTACT: {}", angle_to_contact);
                    let dt = enemy.position - position();
                    set_radar_heading(dt.angle());
                    turn(100.0 * angle_diff(heading(), dt.angle()));
                    accelerate(velocity()*-1.0);
                    if angle_to_contact < 0.03 {
                        fire(0);
                    }
                } else {
                    debug!("NO SHOT")
                }
            }
            None => {
                set_radar_heading(radar_heading() + radar_width());    
            }
        }
        
        
        
    }
}
