use scraper;

pub trait FromElement : Sized
{
    type Err;
    fn from_element(e: &scraper::ElementRef) -> Result<Self, Self::Err>;
}
