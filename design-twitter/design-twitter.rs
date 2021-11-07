use std::collections::HashSet;
#[derive(Debug, Default)]
struct Twitter {
    users: Vec<HashSet<i32>>,
    posts: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            users: vec![HashSet::new(); 501],
            ..Default::default()
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.posts.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut i = 0;
        let mut vec = Vec::with_capacity(10);
        let mut iter = self.posts
            .iter()
            .rev()
            .filter(|(u_id, t_id)| user_id == *u_id || self.users[user_id as usize].contains(u_id))
            .map(|(u_id, t_id)| {*t_id});
        while let Some(x) = iter.next(){
            vec.push(x);
            i+=1;
            if i == 10{
                break;
            }
        }
        vec
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let follower_id = follower_id as usize;
        self.users[follower_id].insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        let follower_id = follower_id as usize;
        self.users[follower_id].remove(&followee_id);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */