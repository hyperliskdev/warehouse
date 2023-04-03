use async_graphql::Object;



pub struct PieceCategory {
    id: i32,
    title: String,
    desc: String,
}

#[Object]
impl PieceCategory {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn title(&self) -> &str {
        &self.title
    }

    async fn desc(&self) -> &str {
        &self.desc
    }
}