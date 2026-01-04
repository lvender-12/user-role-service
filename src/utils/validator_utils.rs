use std::borrow::Cow;

use validator::ValidationError;


//untuk validasi tld email
pub fn validate_email_tld(email:&str)->Result<(), ValidationError>{
    //simple check format email
    let (_, domain) = email.split_once('@')
        .ok_or_else(||ValidationError::new("email tidak valid")
            .with_message(Cow::from("format email tidak falid")))?;

    //check tld email
    let tld = domain.rsplit('.').next()
        .ok_or_else(||ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid")))?;

    //check panjang tld
    if tld.len() < 2 {
        return Err(ValidationError::new("email tidak valid")
            .with_message(Cow::from("Format email tidak valid"))    
        );
    }

    Ok(())
}