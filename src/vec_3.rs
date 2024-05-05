use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;

struct vec3 {
    pub xyz : [f64;3],
}

struct Scalar { value: usize }

impl AddAssign<vec3> for vec3 {
    fn add_assign(&mut self, rhs: &mut vec3) {
        self.xyz[0] += rhs.xyz[0];
        self.xyz[1] += rhs.xyz[1];
        self.xyz[2] += rhs.xyz[2];
    }
}

impl DivAssign<vec3> for vec3 {
    fn div_assign(&mut self, rhs: &mut vec3) {
        self.xyz[0] /= rhs.xyz[0];
        self.xyz[1] /= rhs.xyz[1];
        self.xyz[2] /= rhs.xyz[2];
    }
}

impl Mul<Scalar> for vec3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self { value: self.value.iter().map(|v| v * rhs.value).collect() }
    }
}

impl MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.xyz[0] *= rhs;
        self.xyz[1] *= rhs;
        self.xyz[2] *= rhs;
    }
}


impl vec3 {
    // TODO: do something with these operators
    // vec3() : e{0,0,0} {}
    // vec3(double e0, double e1, double e2) : e{e0, e1, e2} {}
    pub fn x(&self) -> f64 { return self.xyz[0]; }
    pub fn y(&self) -> f64 { return self.xyz[1]; }
    pub fn z(&self) -> f64 { return self.xyz[2]; }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            xyz: [x, y, z]
        }
    }

    // formerly vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
    // double operator[](int i) const { return e[i]; }
    // double& operator[](int i) { return e[i]; }

    // vec3& operator+=(const vec3& v) {
    // e[0] += v.e[0];
    // e[1] += v.e[1];
    // e[2] += v.e[2];
    // return *this;
    // }
    //
    // vec3& operator*=(double t) {
    // e[0] *= t;
    // e[1] *= t;
    // e[2] *= t;
    // return *this;
    // }

    // vec3& operator/=(double t) {
    // return *this *= 1/t;
    // }

    double length() const {
    return sqrt(length_squared());
    }

    double length_squared() const {
    return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
    }
};

// }

// // // point3 is just an alias for vec3, but useful for geometric clarity in the code.
// // using point3 = vec3;


// // Vector Utility Functions

// inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
// return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
// }

// inline vec3 operator+(const vec3& u, const vec3& v) {
// return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
// }

// inline vec3 operator-(const vec3& u, const vec3& v) {
// return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
// }

// inline vec3 operator*(const vec3& u, const vec3& v) {
// return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
// }

// inline vec3 operator*(double t, const vec3& v) {
// return vec3(t*v.e[0], t*v.e[1], t*v.e[2]);
// }

// inline vec3 operator*(const vec3& v, double t) {
// return t * v;
// }

// inline vec3 operator/(const vec3& v, double t) {
// return (1/t) * v;
// }

// inline double dot(const vec3& u, const vec3& v) {
// return u.e[0] * v.e[0]
// + u.e[1] * v.e[1]
// + u.e[2] * v.e[2];
// }

// inline vec3 cross(const vec3& u, const vec3& v) {
// return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
// u.e[2] * v.e[0] - u.e[0] * v.e[2],
// u.e[0] * v.e[1] - u.e[1] * v.e[0]);
// }

// inline vec3 unit_vector(const vec3& v) {
// return v / v.length();
// }
