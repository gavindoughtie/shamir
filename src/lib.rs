pub fn shamir_share(item_count: u32, share_count: u32) {
  println!("Making {} shares of {} items", share_count, item_count);
  let mut share_lists: Vec<Vec<u32>> = vec![];
  let list_items = item_count / share_count;
  for share in 0..share_count {
    let mut list = vec![];
    let first_item = share * list_items;
    let last_item = first_item + list_items;
    for item in first_item..last_item {
      list.push(item);
    }
    share_lists.push(list);
  }
  let mut shares: Vec<Vec<u32>> = vec![];
  for share in 0..share_count {
    let mut share_items = vec![];
    for value in &share_lists[share as usize] {
      share_items.push(*value);
    }
    let next_share: usize = (share as usize) + 1;
    if next_share < share_lists.len() {
      for value in &share_lists[next_share] {
        share_items.push(*value);
      }  
    }
    shares.push(share_items);
  }
  println!("shares: {:?}", shares);
}

#[cfg(test)]
mod tests {
    use crate::shamir_share;

    #[test]
    fn test_split_24_4() {
      shamir_share(24, 3);
    }
}
