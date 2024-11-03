use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub obj: Sphere
}

impl Intersection {
    pub fn new(t: f32, obj: Sphere) -> Self {
        Self {
            t,
            obj
        }
    }
}

pub fn hit(inters: Vec<Intersection>) -> Option<Intersection> {
    let mut min_t = f32::MAX;
    let mut min_inter = None;

    for i in inters {
        if i.t > 0.0 && i.t < min_t {
            min_t = i.t;
            min_inter = Some(i);
        }
    }

    min_inter
}

#[cfg(test)]
mod tests {
    use crate::types::sphere::Sphere;
    use super::{Intersection, hit};

    #[test]
    fn new() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.obj, s);

        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let inter = hit(vec![i1, i2]);
        assert_eq!(inter.unwrap(), i1);

        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let inter = hit(vec![i1, i2]);
        assert_eq!(inter.unwrap(), i2);

        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let inter = hit(vec![i1, i2]);
        assert_eq!(inter, None);

        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let inter = hit(vec![i1, i2, i3, i4]);
        assert_eq!(inter.unwrap(), i4);
    }
}