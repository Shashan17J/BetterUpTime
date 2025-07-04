use poem::FromRequest;

struct UserId(String);

impl<'a> FromRequest<'a> for UserId {
   /// Extract from request head and body.
  async fn from_request(
    req: &'a Request,
    body: &mut RequestBody,
) ->  Result<Self>;
}