use crate::core::collider::Collider;
use crate::core::math_functions::vector_triple_product;
use crate::core::simplex::Simplex;
use nalgebra_glm as glm;
use std::mem;

fn support(collider_a: &Collider, collider_b: &Collider, direction: glm::Vec3) -> glm::Vec3 {
    collider_a.find_furthest_point(direction) - collider_b.find_furthest_point(-direction)
}

fn next_simplex(points: &mut Simplex, direction: &mut glm::Vec3) -> bool {
    return match points.size() {
        2 => line(points, direction),
        3 => triangle(points, direction),
        4 => tetrahedron(points, direction),
        _ => false,
    };
}

fn same_direction(direction: glm::Vec3, ao: glm::Vec3) -> bool {
    glm::dot(&direction, &ao) > 0.0
}

fn line(points: &mut Simplex, direction: &mut glm::Vec3) -> bool {
    let a = points[0];
    let b = points[1];

    let ab = b - a;
    let ao = -a;

    if same_direction(ab, ao) {
        let new_direction = vector_triple_product(&ab, &ao, &ab);
        let _ = mem::replace(direction, new_direction);
    } else {
        points[0] = a;
        points[1] = glm::vec3(0.0, 0.0, 0.0);
        points[2] = glm::vec3(0.0, 0.0, 0.0);
        points[3] = glm::vec3(0.0, 0.0, 0.0);

        let _ = mem::replace(direction, ao);
    }

    false
}

fn triangle(points: &mut Simplex, direction: &mut glm::Vec3) -> bool {
    let a = points[0];
    let b = points[1];
    let c = points[2];

    let ab = b - a;
    let ac = c - a;
    let ao = -a;

    let abc = glm::cross(&ab, &ac);

    if same_direction(glm::cross(&abc, &ao), ao) {
        if same_direction(ac, ao) {
            points[0] = a;
            points[1] = c;
            points[2] = glm::vec3(0.0, 0.0, 0.0);
            points[3] = glm::vec3(0.0, 0.0, 0.0);
            let new_direction = vector_triple_product(&ac, &ao, &ac);
            let _ = mem::replace(direction, new_direction);
        } else {
            return line(&mut Simplex::from_list(vec![a, b]), direction);
        }
    } else {
        if same_direction(glm::cross(&ab, &abc), ao) {
            return line(&mut Simplex::from_list(vec![a, b]), direction);
        } else {
            if same_direction(abc, ao) {
                let _ = mem::replace(direction, abc);
            } else {
                points[0] = a;
                points[1] = b;
                points[2] = c;
                points[3] = glm::vec3(0.0, 0.0, 0.0);

                let _ = mem::replace(direction, -abc);
            }
        }
    }

    false
}

fn tetrahedron(points: &mut Simplex, direction: &mut glm::Vec3) -> bool {
    let a = points[0];
    let b = points[1];
    let c = points[2];
    let d = points[3];

    let ab = b - a;
    let ac = c - a;
    let ad = d - a;
    let ao = -a;

    let abc = glm::cross(&ab, &ac);
    let acd = glm::cross(&ac, &ad);
    let adb = glm::cross(&ad, &ab);

    if same_direction(abc, ao) {
        return triangle(&mut Simplex::from_list(vec![a, b, c]), direction);
    }

    if same_direction(acd, ao) {
        return triangle(&mut Simplex::from_list(vec![a, c, d]), direction);
    }

    if same_direction(adb, ao) {
        return triangle(&mut Simplex::from_list(vec![a, d, b]), direction);
    }

    println!("\n\n\n {}, {}, {}, {}", a, b, c, d);

    true
}

pub(crate) fn intersects(collider_a: &Collider, collider_b: &Collider) -> bool {
    let mut support_vector = support(collider_a, collider_b, glm::zero());
    let mut points = Simplex::empty();
    points.push_front(support_vector);
    let mut direction = -support_vector;

    loop {
        support_vector = support(collider_a, collider_b, direction);

        if glm::dot(&direction, &support_vector) < 0.0 {
            return false;
        }

        points.push_front(support_vector);

        if next_simplex(&mut points, &mut direction) {
            println!("epa {:?}", epa(points.clone(), collider_a, collider_b));
            return true;
        }
    }
}

static TOLERANCE: f32 = 0.00001;

fn epa(
    mut termination_simplex: Simplex,
    collider_a: &Collider,
    collider_b: &Collider,
) -> (glm::Vec3, f32) {
    loop {
        let edge = find_closest_edge(&termination_simplex);

        let p = support(collider_a, collider_b, edge.normal);

        let d = p.dot(&edge.normal);

        if d - edge.distance < TOLERANCE {
            return (edge.normal, d);
        } else {
            termination_simplex.set(edge.index, p);
        }
    }
}

fn find_closest_edge(simplex: &Simplex) -> Edge {
    let distance = f32::MAX;

    let mut closest = Edge {
        distance: 0.0,
        normal: Default::default(),
        index: 0,
    };

    for i in 0..simplex.size() as usize {
        let j = if i + 1 == simplex.size() as usize {
            0
        } else {
            i + 1
        };

        let a = simplex.get(i);
        let b = simplex.get(j);

        let e = glm::vec3(b.x - a.x, b.y - a.y, b.z - a.z);

        let oa = a;

        let n = &vector_triple_product(&e, oa, &e).normalize() as &glm::Vec3;

        let d = n.dot(a);

        if d < distance {
            closest.distance = d;
            closest.normal = *n;
            closest.index = j;
        }
    }

    closest
}

struct Edge {
    distance: f32,
    normal: glm::Vec3,
    index: usize,
}
