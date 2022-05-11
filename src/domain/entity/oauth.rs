pub enum Oauth {
    Password(String, String), // salt, pw_digest
    Wechat,
    Github,
}

