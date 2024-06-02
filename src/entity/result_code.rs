pub struct ResultCode;

impl ResultCode {
    pub fn get_message(code: u32) -> String {
        match code {
            20001 => String::from("用户信息不能为空"),
            20010 => String::from("验证码发送成功"),
            20011 => String::from("验证码发送失败"),
            20012 => String::from("验证码验证失败"),
            20020 => String::from("登陆成功"),
            20021 => String::from("邮箱或密码错误"),
            20030 => String::from("注册成功"),
            20031 => String::from("邮箱已注册"),
            20040 => String::from("身份验证成功"),
            20041 => String::from("身份验证过期"),
            20050 => String::from("登出成功"),
            20051 => String::from("服务器错误"),
            20060 => String::from("更改昵称成功"),
            20061 => String::from("更改昵称失败"),
            20070 => String::from("更改头像成功"),
            20071 => String::from("更改头像失败"),
            20080 => String::from("密码重置成功"),
            20081 => String::from("密码重置失败"),
            20090 => String::from("文章发布成功"),
            20091 => String::from("文章发布失败"),
            20100 => String::from("文章获取成功"),
            20101 => String::from("文章获取失败"),
            20110 => String::from("用户信息获取成功"),
            20111 => String::from("用户信息获取失败"),
            200 => String::from("成功"),
            _ => String::from("未知错误"),
        }
    }
}
