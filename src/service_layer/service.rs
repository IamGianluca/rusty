use crate::{adapters::repository::UserRepository, domain::user::NewUser};

fn create_user<'a>(user: NewUser<'a>, repo: &mut UserRepository<'a>) {
    let _ = repo.save(&user);
}
