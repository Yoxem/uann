(* struct for the pair of matched and remained part to be matched by the parser. *)
type matchee  = { matched : string ; remained : string};;


type 'a option = Just of 'a | Nothing;;

let matchee1 = {matched = ""; remained = "abraca"} ;;


let matchChar ch = fun x ->
  let  remainedHead = (String.sub (x.remained) 0 1) in
  if remainedHead == ch then 
    Just {matched = x.matched ^remainedHead;
          remained = String.sub x.remained 1 ((String.length x.remained) -1)}
  else
    Nothing;;

let a = (matchChar "a") matchee1 in 
match a with
|Just{matched; remained} -> Printf.printf "%s" matched 
|Nothing -> Printf.printf "Nothing";;