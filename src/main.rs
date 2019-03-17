#![allow(unused_variables)]
use typemap::Key;
use typemap::TypeMap;

struct Context<E: Extruder>(E);

trait Extruder {
    fn register(map: &mut TypeMap);
    fn check_dependecies(register: &TypeMap) -> bool;
    fn extrude(map: &mut TypeMap) -> Context<Self>
    where
        Self: Sized;
}

// stop for () context
trait Contextable: Key<Value = Self> + Clone + Sized {}

impl<T: Key<Value = Self> + Clone + Sized> Contextable for T {}

struct VoidValue<X: Key>(X);
impl<T: Contextable> Key for VoidValue<T> {
    type Value = ();
}

impl<T, U> Extruder for (T, U)
where
    T: Contextable,
    U: Contextable,
{
    fn register(register: &mut TypeMap) {
        println!("Handle register");
        register.insert::<VoidValue<T>>(());
        register.insert::<VoidValue<U>>(());
    }

    fn check_dependecies(register: &TypeMap) -> bool {
        println!(
            "Handle check {}, {}",
            register.get::<VoidValue<T>>().is_some(),
            register.get::<VoidValue<U>>().is_some()
        );
        register.get::<VoidValue<T>>().is_some() && register.get::<VoidValue<U>>().is_some()
    }

    fn extrude(map: &mut TypeMap) -> Context<Self> {
        let t = map.get::<T>().unwrap().clone();
        let u = map.get::<U>().unwrap().clone();
        Context((t, u))
    }
}

impl<T> Extruder for (T,)
where
    T: Contextable,
{
    fn register(register: &mut TypeMap) {
        println!("Handle register");
        register.insert::<VoidValue<T>>(());
    }

    fn check_dependecies(register: &TypeMap) -> bool {
        println!("Handle check {}", register.get::<VoidValue<T>>().is_some());
        register.get::<VoidValue<T>>().is_some()
    }

    fn extrude(map: &mut TypeMap) -> Context<Self> {
        let t = map.get::<T>().unwrap().clone();
        Context((t,))
    }
}

impl Extruder for () {
    fn register(_: &mut TypeMap) {}
    fn check_dependecies(_: &TypeMap) -> bool {
        true
    }
    fn extrude(_: &mut TypeMap) -> Context<Self> {
        Context(())
    }
}

trait Handler {
    type Context: Extruder;
    fn handle(&mut self, context: Context<Self::Context>);
}

#[derive(Clone)]
struct Session;
#[derive(Clone)]
struct SomeOther;

impl Key for Session {
    type Value = Session;
}
impl Key for SomeOther {
    type Value = SomeOther;
}

impl Handler for Session {
    type Context = ();
    fn handle(&mut self, context: Context<Self::Context>) {
        unimplemented!()
    }
}

impl Handler for SomeOther {
    type Context = ();
    fn handle(&mut self, context: Context<Self::Context>) {
        unimplemented!()
    }
}

struct Dispatcher {
    registration: TypeMap,
}
impl Dispatcher {
    fn new() -> Dispatcher {
        Dispatcher {
            registration: TypeMap::new(),
        }
    }
    fn register<H: Handler + Contextable>(&mut self, handler: H) {
        assert!(H::Context::check_dependecies(&self.registration));
        self.registration.insert::<VoidValue<H>>(());
    }
}
// user code;
#[derive(Clone)]
struct MyHandler;
impl Handler for MyHandler {
    type Context = (Session, SomeOther);
    fn handle(&mut self, context: Context<Self::Context>) {
        unimplemented!()
    }
}

impl Key for MyHandler {
    type Value = MyHandler;
}

fn main() {
    let mut dispatcher = Dispatcher::new();
    dispatcher.register(Session);

    //    dispatcher.register(MyHandler); // uncomment, to see how it checks dependencies at runtime/
    dispatcher.register(SomeOther);
    dispatcher.register(MyHandler);
}
