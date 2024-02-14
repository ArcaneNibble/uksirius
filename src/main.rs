use std::{
    collections::HashMap,
    error, fmt, io,
    net::UdpSocket,
    sync::{atomic::AtomicBool, Arc},
    thread,
    time::{Duration, SystemTime},
};

use uuid::Uuid;

const SIP_SERV: &'static str = "10.82.0.1";
const SIP_EXT: &'static str = "1000";
const LOCAL_IP: &'static str = "10.82.0.152";

const AUTH_USER: &'static str = "R";
const AUTH_HA1: &'static str = include_str!("auth_dgst");

fn chop_up_res<'a>(resp: &'a str) -> (u32, &'a str, HashMap<&'a str, &'a str>) {
    let mut headers = HashMap::new();
    let mut lines = resp.split("\r\n");

    let status_line = lines.next().unwrap();
    let mut status_split = status_line.splitn(3, " ");

    let status_ver = status_split.next().unwrap();
    assert_eq!(status_ver, "SIP/2.0");
    let status_code = status_split.next().unwrap().parse().unwrap();
    let status_msg = status_split.next().unwrap();

    for hdr_line in lines {
        if hdr_line.starts_with(" ") {
            // fixme totally busted with linebreaks
            todo!();
        }
        if hdr_line == "" {
            break;
        }
        // dbg!(hdr_line);
        let (hdr_key, hdr_val) = hdr_line.split_once(":").unwrap();
        headers.insert(hdr_key.trim(), hdr_val.trim());
    }

    (status_code, status_msg, headers)
}

fn chop_up_req<'a>(req: &'a str) -> (&'a str, &'a str, &'a str, HashMap<&'a str, &'a str>) {
    let mut headers = HashMap::new();
    let (sip, payload) = req.split_once("\r\n\r\n").unwrap();
    // dbg!(sip, payload);

    let mut lines = sip.split("\r\n");

    let req_line = lines.next().unwrap();
    let mut req_split = req_line.splitn(3, " ");

    let req_method = req_split.next().unwrap();
    let req_uri = req_split.next().unwrap();
    let req_ver = req_split.next().unwrap();
    assert_eq!(req_ver, "SIP/2.0");

    for hdr_line in lines {
        if hdr_line.starts_with(" ") {
            // fixme totally busted with linebreaks
            todo!();
        }
        // dbg!(hdr_line);
        let (hdr_key, hdr_val) = hdr_line.split_once(":").unwrap();
        headers.insert(hdr_key.trim(), hdr_val.trim());
    }

    (req_method, req_uri, payload, headers)
}

fn rtp_thread(stop: Arc<AtomicBool>, rtp_sock: UdpSocket) {
    rtp_sock.set_read_timeout(None).unwrap();
    let mut buf = [0; 65536];

    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
        let sz = rtp_sock.recv(&mut buf).unwrap();
        let pkt = &buf[..sz];
        dbg!(pkt);
    }

    println!("rtp thread stopped");
}

#[derive(Debug)]
enum AnyError {
    IoError(io::Error),
    UtfError(std::str::Utf8Error),
}
impl fmt::Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnyError::IoError(e) => e.fmt(f),
            AnyError::UtfError(e) => e.fmt(f),
        }
    }
}
impl error::Error for AnyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            AnyError::IoError(e) => Some(e),
            AnyError::UtfError(e) => Some(e),
        }
    }
}
impl From<io::Error> for AnyError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<std::str::Utf8Error> for AnyError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::UtfError(value)
    }
}

fn main() -> Result<(), AnyError> {
    println!("Hello, world!");

    let sip_sock = UdpSocket::bind((LOCAL_IP, 0))?;
    sip_sock.set_read_timeout(Some(Duration::new(5, 0)))?;
    let sip_local_addr = sip_sock.local_addr()?;
    println!("local sip {}", sip_local_addr);

    let mut buf = [0; 65536];

    let reg_call_id = Uuid::new_v4();
    let reg_from_tag = Uuid::new_v4();
    let register_req_0 = format!(
        "REGISTER sip:{sip_serv} SIP/2.0\r
CSeq: 1 REGISTER\r
Max-Forwards: 70\r
Call-ID: {call_id}\r
Via: SIP/2.0/UDP {sip_local};branch=z9hG4bK-{branch}\r
From: <sip:{ext}@{sip_serv}>;tag={from_tag}\r
To: <sip:{ext}@{sip_serv}>\r
Contact: <sip:{ext}@{sip_local}>\r
\r\n",
        sip_serv = SIP_SERV,
        ext = SIP_EXT,
        sip_local = sip_local_addr,
        call_id = &reg_call_id,
        branch = Uuid::new_v4(),
        from_tag = &reg_from_tag,
    );
    dbg!(&register_req_0);
    println!("{}", register_req_0);

    sip_sock.send_to(register_req_0.as_bytes(), (SIP_SERV, 5060))?;

    let (sz, _) = sip_sock.recv_from(&mut buf)?;
    let register_resp_0 = std::str::from_utf8(&buf[..sz])?;
    println!("~~~~~");
    println!("{}", register_resp_0);
    let (resp0_status, _, resp0_hdrs) = chop_up_res(register_resp_0);
    assert_eq!(resp0_status, 401);
    assert_eq!(resp0_hdrs.get("Call-ID").unwrap(), &reg_call_id.to_string());
    let auth = resp0_hdrs.get("WWW-Authenticate").unwrap();
    println!("authenticate {}", auth);
    assert!(auth.starts_with("Digest "));
    let mut auth_bits = HashMap::new();
    for auth_bit in auth.split_at(7).1.split(",") {
        // dbg!(auth_bit);
        let (auth_bit_k, auth_bit_v) = auth_bit.split_once("=").unwrap();
        auth_bits.insert(auth_bit_k.trim(), auth_bit_v.trim());
    }
    dbg!(&auth_bits);
    assert_eq!(auth_bits.get("algorithm").unwrap(), &"MD5");
    assert_eq!(auth_bits.get("qop").unwrap(), &"\"auth\"");
    let realm = auth_bits.get("realm").unwrap();
    assert!(realm.starts_with("\""));
    assert!(realm.ends_with("\""));
    let realm = realm.split_at(1).1;
    let realm = realm.split_at(realm.len() - 1).0;
    assert_eq!(realm, "asterisk");
    let nonce = auth_bits.get("nonce").unwrap();
    assert!(nonce.starts_with("\""));
    assert!(nonce.ends_with("\""));
    let nonce = nonce.split_at(1).1;
    let nonce = nonce.split_at(nonce.len() - 1).0;
    let opaque = auth_bits.get("opaque").unwrap();
    assert!(opaque.starts_with("\""));
    assert!(opaque.ends_with("\""));
    let opaque = opaque.split_at(1).1;
    let opaque = opaque.split_at(opaque.len() - 1).0;
    let ha2_data = format!("REGISTER:sip:{}", SIP_SERV);
    dbg!(&ha2_data);
    let ha2 = md5::compute(ha2_data.as_bytes());
    let auth_resp_data = format!("{}:{}:00000001:a:auth:{:x}", AUTH_HA1, nonce, ha2);
    dbg!(&auth_resp_data);
    let auth_resp = md5::compute(auth_resp_data.as_bytes());

    let register_req_1 = format!(
        "REGISTER sip:{sip_serv} SIP/2.0\r
CSeq: 2 REGISTER\r
Max-Forwards: 70\r
Call-ID: {call_id}\r
Via: SIP/2.0/UDP {sip_local};branch=z9hG4bK-{branch}\r
From: <sip:{ext}@{sip_serv}>;tag={from_tag}\r
To: <sip:{ext}@{sip_serv}>\r
Contact: <sip:{ext}@{sip_local}>\r
Authorization: Digest username=\"{username}\",realm=\"{realm}\",nonce=\"{nonce}\",uri=\"sip:{sip_serv}\",opaque=\"{opaque}\",algorithm=MD5,qop=\"auth\",nc=00000001,cnonce=\"a\",response=\"{auth_resp:x}\"\r
\r\n",
        sip_serv = SIP_SERV,
        ext = SIP_EXT,
        sip_local = sip_local_addr,
        call_id = &reg_call_id,
        branch = Uuid::new_v4(),
        from_tag = &reg_from_tag,
        username=AUTH_USER,
    );
    dbg!(&register_req_1);
    println!("{}", register_req_1);

    sip_sock.send_to(register_req_1.as_bytes(), (SIP_SERV, 5060))?;

    let (sz, _) = sip_sock.recv_from(&mut buf)?;
    let register_resp_1 = std::str::from_utf8(&buf[..sz])?;
    println!("~~~~~");
    println!("{}", register_resp_1);
    let (resp1_status, _, resp1_hdrs) = chop_up_res(register_resp_1);
    assert_eq!(resp1_status, 200);
    assert_eq!(resp1_hdrs.get("Call-ID").unwrap(), &reg_call_id.to_string());

    sip_sock.set_read_timeout(None)?;

    let mut our_call_id = None;

    let run_rtp_thread = Arc::new(AtomicBool::new(false));

    loop {
        let (sz, _) = sip_sock.recv_from(&mut buf)?;
        let sip_req = std::str::from_utf8(&buf[..sz])?;
        println!("~~~~~");
        println!("{}", sip_req);

        let (req_method, req_uri, req_payload, req_headers) = chop_up_req(sip_req);

        if req_method == "INVITE" {
            assert_eq!(req_uri, format!("sip:{}@{}", SIP_EXT, sip_local_addr));
            assert_eq!(req_headers.get("Content-Type").unwrap(), &"application/sdp");
            let content_len = req_headers
                .get("Content-Length")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            assert!(req_payload.len() >= content_len);

            let _invite_payload = &req_payload[..content_len];

            our_call_id = Some(req_headers.get("Call-ID").unwrap().to_string());

            let mut pbx_rtp_addr = None;
            let mut pbx_rtp_port = None;

            for sdp_line in req_payload.lines() {
                dbg!(&sdp_line);
                if sdp_line.starts_with("c=") {
                    assert!(sdp_line.starts_with("c=IN IP4 "));
                    assert!(pbx_rtp_addr.is_none());
                    pbx_rtp_addr = Some(sdp_line.split_at(9).1.trim().to_string());
                } else if sdp_line.starts_with("m=") {
                    assert!(pbx_rtp_port.is_none());
                    let media_bits = sdp_line.split_at(2).1.split(" ").collect::<Vec<_>>();
                    assert_eq!(media_bits[0], "audio");
                    assert_eq!(media_bits[2], "RTP/AVP");
                    assert!(media_bits[3..].contains(&"0"));
                    pbx_rtp_port = Some(media_bits[1]);
                }
            }

            let pbx_rtp = format!("{}:{}", pbx_rtp_addr.unwrap(), pbx_rtp_port.unwrap());
            println!("rtp for pbx is {}", pbx_rtp);

            // rtp setup for our side
            let rtp_sock = UdpSocket::bind((LOCAL_IP, 0))?;
            let rtp_local_addr = rtp_sock.local_addr()?;
            rtp_sock.connect(pbx_rtp)?;
            println!("local rtp {}", rtp_local_addr);

            let answered_sdp = format!(
                "v=0\r
o=- {ts} {ts} IN IP4 {our_ip}\r
s=-\r
c=IN IP4 {our_ip}\r
t=0 0\r
m=audio {our_port} RTP/AVP 0\r\n",
                ts = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                our_ip = LOCAL_IP,
                our_port = rtp_local_addr.port(),
            );

            let answered_okay = format!(
                "SIP/2.0 200 OK\r
CSeq: {cseq}\r
Call-ID: {call_id}\r
Via: {via}\r
From: {from}\r
To: {to};tag={to_tag}\r
Content-Type: application/sdp\r
\r
{payload}",
                cseq = req_headers.get("CSeq").unwrap(),
                call_id = req_headers.get("Call-ID").unwrap(),
                via = req_headers.get("Via").unwrap(),
                from = req_headers.get("From").unwrap(),
                to = req_headers.get("To").unwrap(),
                to_tag = Uuid::new_v4(),
                payload = answered_sdp,
            );
            println!("{}", answered_okay);

            sip_sock.send_to(answered_okay.as_bytes(), (SIP_SERV, 5060))?;

            let run_rtp_thread_clone = run_rtp_thread.clone();
            thread::spawn(move || {
                rtp_thread(run_rtp_thread_clone, rtp_sock);
            });

            println!("***** WE ANSWERED CALL *****");
        } else if req_method == "BYE" {
            assert_eq!(req_uri, format!("sip:{}@{}", SIP_EXT, sip_local_addr));
            if let Some(our_call_id) = &our_call_id {
                if req_headers.get("Call-ID").unwrap() == our_call_id {
                    println!("***** CALL HANG UP *****");

                    run_rtp_thread.store(true, std::sync::atomic::Ordering::Relaxed);

                    let answered_bye = format!(
                        "SIP/2.0 200 OK\r
CSeq: {cseq}\r
Call-ID: {call_id}\r
Via: {via}\r
From: {from}\r
To: {to}\r
\r\n",
                        cseq = req_headers.get("CSeq").unwrap(),
                        call_id = req_headers.get("Call-ID").unwrap(),
                        via = req_headers.get("Via").unwrap(),
                        from = req_headers.get("From").unwrap(),
                        to = req_headers.get("To").unwrap(),
                    );
                    println!("{}", answered_bye);

                    sip_sock.send_to(answered_bye.as_bytes(), (SIP_SERV, 5060))?;

                    break;
                }
            } else {
                // dunno

                let dunno_bye = format!(
                    "SIP/2.0 500 Internal Server Error\r
CSeq: {cseq}\r
Call-ID: {call_id}\r
Via: {via}\r
From: {from}\r
To: {to}\r
\r\n",
                    cseq = req_headers.get("CSeq").unwrap(),
                    call_id = req_headers.get("Call-ID").unwrap(),
                    via = req_headers.get("Via").unwrap(),
                    from = req_headers.get("From").unwrap(),
                    to = req_headers.get("To").unwrap(),
                );
                println!("{}", dunno_bye);

                sip_sock.send_to(dunno_bye.as_bytes(), (SIP_SERV, 5060))?;
            }
        } else if req_method == "ACK" {
            println!("yeah we should handle this or something");
        } else {
            // dunno

            let dunno_resp = format!(
                "SIP/2.0 500 Internal Server Error\r
CSeq: {cseq}\r
Call-ID: {call_id}\r
Via: {via}\r
From: {from}\r
To: {to}\r
\r\n",
                cseq = req_headers.get("CSeq").unwrap(),
                call_id = req_headers.get("Call-ID").unwrap(),
                via = req_headers.get("Via").unwrap(),
                from = req_headers.get("From").unwrap(),
                to = req_headers.get("To").unwrap(),
            );
            println!("{}", dunno_resp);

            sip_sock.send_to(dunno_resp.as_bytes(), (SIP_SERV, 5060))?;
        }
    }

    Ok(())
}
