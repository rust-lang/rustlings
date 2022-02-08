/* define your structure here


*/

fn isEmpty(???) -> bool {
   
}

fn length(???, ???) -> i32 {
   
}


fn displayForward(???) {

}

fn displayBackward(???) {
	
}

fn insertFirst(???, key: i32, data: i32) {

}

fn insertLast(???, key: i32, data: i32) {

}

fn deleteFirst(???, ???) -> ???{

}


fn deleteLast(???, ???) -> ??? {

}

fn delete(???, ???, ???, key: i32) -> ??? {

}

fn insertAfter(???, ???, ???, key: i32, newKey: i32, data: i32) -> bool{

}

void main() {
   /* declare your nodes here for head, last, and current


   */

   insertFirst(???, 1, 10);
   insertFirst(???, 2, 20);
   insertFirst(???, 3, 30);
   insertFirst(???, 4, 1);
   insertFirst(???, 5, 40);
   insertFirst(???, 6, 56); 

   println!("\nList (First to Last): ");  
   displayForward(???);
	
   println!("\n");
   println!("\nList (Last to first): "); 
   displayBackward(???);

   println!("\nList , after deleting first record: ");
   deleteFirst(???, ???);        
   displayForward(???);

   println!("\nList , after deleting last record: ");  
   deleteLast(???, ???);
   displayForward(???);

   println!("\nList , insert after key(4) : ");  
   insertAfter(???, ???, ???, 4, 7, 13);
   displayForward(???);

   println!("\nList  , after delete key(4) : ");  
   delete(???, ???, ???, 4);
   displayForward(???);
}