//نحتاج استيراد التالي حتى يمكن لبيئة العمل ترجمة السطور التالية و تحويلها لكود يعمل على البلوك تشين
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

near_sdk::setup_alloc!();
//ماكرو حتى يمكن لبيئة العمل ترجمة الكود التالي الى كود يعمل على البلوك تشين
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

//المكون الرئيسي للعقد الذكي - ستركتشر
pub struct Contract {
  //يمكن تعريف المتغيرات هنا
}

//الدوال الملحقة بالاستركتشر السابق تعريفه
#[near_bindgen]
impl Contract {
  //تعريف الدوال هنا
  pub fn hello_world(&self) -> std::string::String {
    return "Hello from Rust!".to_string();
  }
}
