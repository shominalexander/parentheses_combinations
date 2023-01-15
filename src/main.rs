fn repeats(char: char, text: &String) -> usize {
 let mut repeats: usize = 0usize;

 for text_char in text.chars() {
  if text_char == char {
   repeats += 1;

  }//if text_char == char {
 }//for text_char in text.chars() {

 repeats
}//fn repeats(char: char, text: &String) -> usize {

fn size(strings: &Vec<String>) -> usize {
 let mut size: usize = 0;

 for string in strings {
  let length: usize = string.len();

  if length > size {
   size = length;

  }//if length > size {
 }//for string in strings {

 size
}//fn size(strings: &Vec<String>) -> usize {

fn level(amount: usize, variants: Vec<String>) -> Vec<String> {
 let mut strings: Vec<String> = vec![];

 if variants.len() == 0 {
  strings.push("(".to_string());

 } else {//if variants.len() == 0 {
  for variant in &variants {
   let closing: usize = repeats(')', variant);
   let open   : usize = repeats('(', variant);

   if closing < amount && closing < open {
    let string: String = variant.to_string() + ")";

    strings.push(string);
   }//if closing < amount && closing < open {

   if open < amount {
    let string: String = variant.to_string() + "(";

    strings.push(string);
   }//if open < amount {
  }//for variant in &variants {
 }//} else {//if strings.len() == 0 {

 let size: usize = size(&strings);

 if size == amount * 2 {
  strings

 } else {//if size == amount * 2 {
  level(amount, strings)

 }//} else {//if size == amount * 2 {
}//fn level(amount: usize, variants: Vec<String>) -> Vec<String> {

fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn variants (text: String) -> Vec<String> {
 let mut variants: Vec<String> = vec![];

 match text.parse::<usize>() {
  Ok(amount) => { variants = level(amount, variants); }
  Err(error) => {            println!("{:?}", error); }
 }//match text.parse::<usize>() {

 variants.sort();

 variants
}//fn variants (text: String) -> Vec<String> {

fn main(){
 loop {
  println!("\r\n\r\namount:"); 

  let amount: String = request();
 
  if &amount[..] == "exit" {
   break;   

  } else {//if &amount[..] == "exit" {
   let variant: Vec<String> = variants(amount);

   println!("variant: {:?}", variant);
  }//} else {//if &amount[..] == "exit" {
 }//loop {
}//fn main(){
