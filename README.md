Structs and methods for linear algebra.

This crate provides 3 main types of structs that implement basic arithmetic operations 
like + and -, and also pervide useful methods:

- Structs for handling vectors like [Vec2](crate::vec2::Vec2) and [Vec3](crate::vec3::Vec3).
- Structs for handling arrays of vectors with different types of memory orginization, like
  [Vec3arr](crate::vec3arr::Vec3arr) and [Vec3box](crate::vec3arr::Vec3box), for fixed length
  arrays and boxed arrays respectively.
- Structs for handling square matricies like [Mat22](crate::mat22::Mat22) and
  [Mat33](crate::mat33::Mat33).

Vectors can be easily initialized and used:
```rust
# extern crate lineq
use lineq::vec3::Vec3;
let a : Vec3 = Vec3::UP;
let b : Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
assert_eq!(a.cross(b),Vec3::IN);
```

Arrays of vectors can be used to parallelize arithmetic:
```rust
# extern crate lineq
use lineq::vec3::Vec3;
use lineq::vec3arr::Vec3arr;
//using a and b from last example:
# let a : Vec3 = Vec3::UP;
# let b : Vec3 = Vec3::LEFT;
let ab : Vec3arr = Vec3arr([a,b]);
let cd : Vec3arr = Vec3arr([Vec3::DOWN,Vec3::RIGHT]);
assert_eq!(ab + cd, Vec3arr([Vec3::ZERO,Vec3::ZERO]));
```

Matricies are indexed like x1, y2, z3 ... where x, y, z
are the rows and 1, 2, 3 are the columns:
```rust
# extern crate lineq
use lineq::vec3::Vec3;
use lineq::mat33::Mat33;
//using a and b from first example:
# let a : Vec3 = Vec3::UP;
# let b : Vec3 = Vec3::LEFT;
let c : Vec3 = Vec3{ x: 1.0, y: -1.0, z: 2.0 };
let m1 : Mat33 = Mat33::augment(a,b,c);
let m2 : Mat33 = Mat33{
x1: 0.0, x2: -1.0, x3: 1.0,
y1: 1.0, y2: 0.0, y3: -1.0,
z1: 0.0, z2: 0.0, z3: 2.0 }
assert_eq!(m1,m2);
```
