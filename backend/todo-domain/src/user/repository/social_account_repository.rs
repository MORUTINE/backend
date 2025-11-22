use crate::user::models::oauth_provider::OAuthProvider;
use crate::user::models::social_account::SocialAccount;

pub trait SocialAccountRepository {
    /// 로그인용 — 소셜 인증 → 내부 회원 찾기
    fn find_by_provider_and_user_id(
        &self,
        provider: OAuthProvider,
        provider_user_id: &str,
    ) -> Result<Option<SocialAccount>, anyhow::Error>;

    /// 계정 연동/조회용
    fn find_by_user_id_and_provider(
        &self,
        user_id: i64,
        provider: &OAuthProvider,
    ) -> Result<Option<SocialAccount>, anyhow::Error>;

    fn save(&self, social_account: SocialAccount) -> Result<SocialAccount, anyhow::Error>;
}
