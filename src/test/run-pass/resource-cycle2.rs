// Don't leak the unique pointers

type u = {
    a: int,
    b: int,
    c: *int
};

resource r(v: u) unsafe {
    let v2: ~int = unsafe::reinterpret_cast(v.c);
}

enum t = {
    mut next: option<@t>,
    r: r
};

fn main() unsafe {
    let i1 = ~0xA;
    let i1p = unsafe::reinterpret_cast(i1);
    unsafe::forget(i1);
    let i2 = ~0xA;
    let i2p = unsafe::reinterpret_cast(i2);
    unsafe::forget(i2);

    let u1 = {a: 0xB, b: 0xC, c: i1p};
    let u2 = {a: 0xB, b: 0xC, c: i2p};

    let x1 = @t({
        mut next: none,
        r: r(u1)
    });
    let x2 = @t({
        mut next: none,
        r: r(u2)
    });
    x1.next = some(x2);
    x2.next = some(x1);
}
