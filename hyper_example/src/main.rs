use digest_auth::AuthContext;

// Value from the WWW-Authenticate HTTP header (usually in a HTTP 401 response)
let www_authenticate = r#"Digest username="root", realm="AXIS_ACCC8EB4F9B5", nonce="rhK/AhceBQA=80f2fe4749dbab8697f4a6aef0d976555afdd5f3", uri="http://192.168.1.193/axis-cgi/param.cgi?action=list&responseformat=rfc", algorithm=MD5, response="926701bef60fc8f2a719f6aba3d13ff0", qop=auth, nc=00000002, cnonce="800637a8fcbaa59f""#;

// Prepare an authorization context. Note that this is a GET request. There are different
// constructors available for POST or other request types. You can re-use it, but
// it's cheap to create a fresh one each time, as the struct uses references only.
let mut context = AuthContext::new("Mufasa", "Circle of Life", "/dir/index.html");
// For this test, we inject a custom cnonce. It's generated for you otherwise
// - you don't need `mut` in that case and needn't worry about this at all.
context.set_custom_cnonce("f2/wE4q74E6zIJEtWaHKaf5wv/H5QzzpXusqGemxURZJ");

// Parse the prompt header. You can inspect the parsed object, its fields are public.
let mut prompt = digest_auth::parse(www_authenticate).unwrap();

// Compute a value for the Authorization header that we'll send back to the server
let answer = prompt.respond(&context).unwrap().to_string();
assert_eq!(answer, r#"Digest username="root", realm="AXIS_ACCC8EB4F9B5", nonce="rhK/AhceBQA=80f2fe4749dbab8697f4a6aef0d976555afdd5f3", uri="http://192.168.1.193/axis-cgi/param.cgi?action=list&responseformat=rfc", algorithm=MD5, response="926701bef60fc8f2a719f6aba3d13ff0", qop=auth, nc=00000002, cnonce="800637a8fcbaa59f""#);

// The `prompt` variable is mutable, because the 'nc' counter (nonce reuse count)
// is inside the struct and updated automatically.

// You can re-use it for subsequent requests, assuming the server allows nonce re-use.
// Some poorly implemented servers will reject it and give you 401 again, in which case
// you should parse the new "WWW-Authenticate" header and use that instead.

let answer2 = prompt.respond(&context).unwrap().to_string();
// notice how the 'response' field changed - the 'nc' counter is included in the hash
assert_eq!(answer2, r#"Digest username="root", realm="AXIS_ACCC8EB4F9B5", nonce="rhK/AhceBQA=80f2fe4749dbab8697f4a6aef0d976555afdd5f3", uri="http://192.168.1.193/axis-cgi/param.cgi?action=list&responseformat=rfc", algorithm=MD5, response="926701bef60fc8f2a719f6aba3d13ff0", qop=auth, nc=00000002, cnonce="800637a8fcbaa59f""#);