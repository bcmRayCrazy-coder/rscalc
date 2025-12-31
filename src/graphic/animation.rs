use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub enum Animated<T> {
    /**
     * Animating(value: T, target: T, speed: T)
     */
    Animating(T, T, T),

    /**
     * Animate done(value: T)
     */
    Done(T),
}

impl<T> Animated<T> {
    pub fn done_to(target: T) -> Self {
        Animated::Done(target)
    }
}

impl<T: Clone> Animated<T> {
    pub fn animate_to(&self, target: T, speed: T) -> Self {
        match self {
            Animated::Animating(val, _, _) => Animated::Animating(val.clone(), target, speed),
            Animated::Done(val) => Animated::Animating(val.clone(), target, speed),
        }
    }
}

impl Animated<f32> {
    pub fn ensure_frame(&mut self, frame_time: f32) -> &f32 {
        if let Animated::Animating(val, target, speed) = &self {
            let new_val = val + speed * frame_time;
            let normalized_speed = speed / speed;
            if (target - new_val) * normalized_speed <= 0.0 {
                *self = Animated::Done(val.clone());
            }
        }
        match self {
            Animated::Animating(val, _, _) => val,
            Animated::Done(val) => val,
        }
    }
}

impl Animated<i32> {
    pub fn ensure_frame(&mut self, frame_time: f32) -> &i32 {
        if let Animated::Animating(val, target, speed) = &self {
            let new_val = val + speed * frame_time.round() as i32;
            let normalized_speed = speed / speed;
            if (target - new_val) * normalized_speed <= 0 {
                *self = Animated::Done(val.clone());
            }
        }
        match self {
            Animated::Animating(val, _, _) => val,
            Animated::Done(val) => val,
        }
    }
}

impl Animated<Vec2> {
    pub fn ensure_frame(&mut self, frame_time: f32) -> &Vec2 {
        if let Animated::Animating(val, target, speed) = &self {
            let new_val = val + speed * frame_time;
            let normalized_speed = speed / speed;
            if (target - new_val).x * normalized_speed.x <= 0.0 {
                *self = Animated::Done(val.clone());
            }
        }
        match self {
            Animated::Animating(val, _, _) => val,
            Animated::Done(val) => val,
        }
    }
}

impl Animated<Vec3> {
    pub fn ensure_frame(&mut self, frame_time: f32) -> &Vec3 {
        if let Animated::Animating(val, target, speed) = &self {
            let new_val = val + speed * frame_time;
            let normalized_speed = speed / speed;
            if (target - new_val).x * normalized_speed.x <= 0.0 {
                *self = Animated::Done(val.clone());
            }
        }
        match self {
            Animated::Animating(val, _, _) => val,
            Animated::Done(val) => val,
        }
    }
}
