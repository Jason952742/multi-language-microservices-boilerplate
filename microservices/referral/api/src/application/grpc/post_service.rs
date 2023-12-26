use tonic::{Request, Response, Status};

use crate::domain::entity::post;
use sea_orm::{DatabaseConnection};
use crate::infra::repo::mutation::Mutation;
use crate::infra::repo::query::Query;


pub mod post_mod {
    tonic::include_proto!("post");
}

use post_mod::{
    blogpost_server::{Blogpost},
    Post, PostId, PostList, PostPerPage, ProcessStatus,
};


impl Post {
    fn into_model(self) -> post::Model {
        post::Model {
            id: self.id,
            title: self.title,
            text: self.content,
        }
    }
}

#[derive(Default)]
pub struct MyServer {
    pub connection: DatabaseConnection,
}

#[tonic::async_trait]
impl Blogpost for MyServer {
    async fn get_posts(&self, request: Request<PostPerPage>) -> Result<Response<PostList>, Status> {
        let conn = &self.connection;
        let posts_per_page = request.into_inner().per_page;

        let mut response = PostList { post: Vec::new() };

        let (posts, _) = Query::find_posts_in_page(conn, 1, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        for post in posts {
            response.post.push(Post {
                id: post.id,
                title: post.title,
                content: post.text,
            });
        }

        Ok(Response::new(response))
    }

    async fn add_post(&self, request: Request<Post>) -> Result<Response<PostId>, Status> {
        let conn = &self.connection;

        let input = request.into_inner().into_model();

        let inserted = Mutation::create_post(conn, input)
            .await
            .expect("could not insert post");

        let response = PostId {
            id: inserted.id.unwrap(),
        };

        Ok(Response::new(response))
    }

    async fn update_post(&self, request: Request<Post>) -> Result<Response<ProcessStatus>, Status> {
        let conn = &self.connection;
        let input = request.into_inner().into_model();

        match Mutation::update_post_by_id(conn, input.id, input).await {
            Ok(_) => Ok(Response::new(ProcessStatus { success: true })),
            Err(_) => Ok(Response::new(ProcessStatus { success: false })),
        }
    }

    async fn delete_post(
        &self,
        request: Request<PostId>,
    ) -> Result<Response<ProcessStatus>, Status> {
        let conn = &self.connection;
        let id = request.into_inner().id;

        match Mutation::delete_post(conn, id).await {
            Ok(_) => Ok(Response::new(ProcessStatus { success: true })),
            Err(_) => Ok(Response::new(ProcessStatus { success: false })),
        }
    }

    async fn get_post_by_id(&self, request: Request<PostId>) -> Result<Response<Post>, Status> {
        let conn = &self.connection;
        let id = request.into_inner().id;

        if let Some(post) = Query::find_post_by_id(conn, id).await.ok().flatten() {
            Ok(Response::new(Post {
                id,
                title: post.title,
                content: post.text,
            }))
        } else {
            Err(Status::new(
                tonic::Code::Aborted,
                "Could not find post with id ".to_owned() + &id.to_string(),
            ))
        }
    }
}
