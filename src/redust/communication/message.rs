use uuid::Uuid;

trait IMessage<'t> {
    fn new(id: Uuid, payload: &'t [u8]) -> Self;
    fn get_id(&self) -> Uuid;
    fn get_payload(&self) -> &'t [u8];
}

struct User<'t> {
    id: Uuid,
    payload: &'t [u8],
}

impl<'t> IMessage<'t> for User<'t> {
    fn new(id: uuid::Uuid, payload: &'t [u8]) -> User<'t> {
        User::<'t> { id, payload }
    }
    fn get_id(&self) -> uuid::Uuid {
        self.id
    }
    fn get_payload(&self) -> &'t [u8] {
        self.payload
    }
}

#[test]
fn test_message() {
    let buffer = [0; 1024];

    let me = User::new(Uuid::new_v4(), &buffer);

    println!("id : {}", me.get_id());
}
