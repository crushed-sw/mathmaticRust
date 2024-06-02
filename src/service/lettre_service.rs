use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use crate::util::error::Error;
use rand::Rng;

pub struct LettreService;

impl LettreService {
    pub async fn send_code(code: &String, to: &String) -> Result<(), Box<dyn std::error::Error>> {
        if code.is_empty() || to.is_empty() {
            return Err(Box::new(Error::from("")));
        }

        let from = "1138775977@qq.com";
        let email = Message::builder()
            .from(from.parse()?)
            .to(to.parse()?)
            .subject("邮箱验证")
            .body(format!("邮箱验证码 {}, 30分钟后验证码失效，如果这封邮件不是你的操作，请忽略。", code.as_str()))?;

        let smtp_server = "smtp.qq.com";
        let smtp_username = "1138775977@qq.com";
        let smtp_password = "vxkcyzbtejaibadc";

        let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

        let mailer = SmtpTransport::relay(smtp_server)?
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(_) => Err(Box::new(Error::from(""))),
        }
    }

    pub async fn get_and_send_code(to: &String) -> Result<String, Error> {
        let mut rng = rand::thread_rng();
        let code = rng.gen_range(0..10000);
        let code: String = format!("{:04}", code);

        let res = Self::send_code(&code, &to).await;
        match res {
            Ok(_) => Ok(code),
            Err(_) => Err(Error::from("验证码发送失败，请稍后再试")),
        }
    }
}
