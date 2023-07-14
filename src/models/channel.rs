use uuid::Uuid;

use super::user::User;

#[derive(Clone)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<User>   
}

impl Channel {
    pub fn get_users(&self) {
        let mut result = String::from("Users in channel: ");
        for user in self.users.iter() {
            result.push_str(&format!("{}, ", user.name));
        }
        result.pop();
        result.pop();
    }

    pub fn add_user(&mut self, user: &User) {
        self.users.push(user.clone());
        
    }

    pub fn remove_user(&mut self, user: &User) {
        self.users = self.users.iter().filter(|&u| u.name != user.name).cloned().collect();
    }


    pub fn message(&self, message: String) {
        for user in self.users.iter() {
            // TODO : Send message to user
            // Needs SSE 
        }
    }
}

