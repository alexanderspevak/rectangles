
type Row=usize;
type Column=usize;


#[derive(Clone,Debug)]
enum SquarePart {
    Corner(Row,Column),
    Horizontal(Row,Column),
    Vertical(Row,Column)
}

type LeftCorner=usize;
type RightCorner=usize;
type TopCorner=usize;
type BottomCorner=usize;
type HorizontalLine=(Row,LeftCorner,RightCorner);
type VerticalLine=(Column,TopCorner,BottomCorner);
#[derive(Debug)]
struct Square {
    top_line:HorizontalLine,
    bottom_line:HorizontalLine
}

fn get_all_square_parts(lines: &[&str])->Vec<SquarePart>{
    let mut coords:Vec<SquarePart>=vec![];
    for (row,line) in lines.into_iter().enumerate(){
        for (column,char) in line.chars().enumerate(){
            match char {
                '+'=>coords.push(SquarePart::Corner(row, column)),
                '-'=>coords.push(SquarePart::Horizontal(row, column)),
                '|'=>coords.push(SquarePart::Vertical(row, column)),
                _=>{}
            }
        }
    }
    return coords
}

fn get_square_part(square_parts:&[SquarePart],requested_row:usize,requested_column:usize)->Option<SquarePart>{
   return  square_parts.iter().cloned().find(|square_part|{
        match square_part {
            SquarePart::Corner(row,column )=>*row==requested_row&& *column==requested_column,
            SquarePart::Horizontal(row,column )=>*row==requested_row&& *column==requested_column,
            SquarePart::Vertical(row,column )=>*row==requested_row&& *column==requested_column,
        }
    })
}

fn get_square_dots_in_line(square_parts:&[SquarePart],row:usize)->Option<Vec<usize>>{
    let mut dots=Vec::new();
    square_parts.iter().for_each(|square_part|{
        if let SquarePart::Corner(square_row,square_column )=square_part{
            if row ==*square_row {
                dots.push(*square_column)
            }
        }
    });
    
    return Some(dots)
}

fn get_line_combos(square_parts:&[SquarePart],lines_len:usize)->Vec<HorizontalLine>{
    let mut line_combos: Vec<(usize, usize, usize)>=Vec::new();
     for line in 0..lines_len {
        if let Some(dots)=get_square_dots_in_line(square_parts, line){
            for (idx,column) in dots.iter().enumerate() {
                let mut next_idx=idx+1;
                while let Some(next_dot_column)=dots.get(next_idx){
                    line_combos.push((line,*column,*next_dot_column));
                    next_idx+=1;
                }
            }
        }
    }

    return line_combos
}

fn get_squares(square_parts:&[SquarePart],lines_len:usize)->Vec<Square>{
    let line_combos=get_line_combos(square_parts,lines_len);
    let mut candidate_squares=Vec::new();
    
    for (idx,(row,left_column,right_column)) in line_combos.iter().enumerate() {
        let mut next_idx=idx+1;
        while let Some((bottom_row,bottom_left_column,bottom_right_column))=line_combos.get(next_idx){
            next_idx+=1;

            if bottom_row==row {
                continue;
            }

            if *left_column==*bottom_left_column && *right_column==*bottom_right_column{
                candidate_squares.push(Square { top_line: (*row,*left_column,*right_column), bottom_line: (*bottom_row,*bottom_left_column,*bottom_right_column) })
            }
        }
    }

    return candidate_squares
}

fn is_vertical_line_valid(square_parts:&[SquarePart],(column,top_corner,bottom_corner):VerticalLine)->bool{
    for row in top_corner..bottom_corner {
        match get_square_part(square_parts,row,column){
            Some(square_part)=>{
                if let  SquarePart::Horizontal(..)=square_part {
                    return false
                }
            }
            None=>return false
        }
    }
    return true
}

fn is_horizontal_line_valid(square_parts:&[SquarePart],(row,left_corner,right_corner):HorizontalLine)->bool {
    for column in left_corner..right_corner {
        match get_square_part(square_parts,row,column){
            Some(square_part)=>{
                if let  SquarePart::Vertical(..)=square_part {
                    return false
                }
            }
            None=>return false
        }
    }
    return true
}

fn is_square_valid(square_parts:&[SquarePart],square:&Square)->bool{
    let (top_row,top_left_corner,top_right_corner)=square.top_line;
    let (bottom_row,botom_left_corner,bottom_right_corner)=square.bottom_line;

    let is_top_line_valid=is_horizontal_line_valid(square_parts,( top_row,top_left_corner,top_right_corner));
    let is_bottom_line_valid=is_horizontal_line_valid(square_parts,(bottom_row,botom_left_corner,bottom_right_corner));

    let is_left_line_valid=is_vertical_line_valid(square_parts,(top_left_corner, top_row,bottom_row));
    let is_right_line_valid=is_vertical_line_valid(square_parts,(top_right_corner, top_row,bottom_row));


    return is_bottom_line_valid&& is_top_line_valid&& is_left_line_valid&& is_right_line_valid
}


fn count_valid_squares(square_parts:&[SquarePart],squares:Vec<Square>)->usize{
    return squares.iter().fold(0,|mut acc,square|{
        if is_square_valid(square_parts, square){
            acc+=1;
        }
        return acc
    })
}

fn count(lines: &[&str])->u32{
    if lines.len()==0{
        return 0
    }

    let square_parts=get_all_square_parts(lines);
    let squares=get_squares(&square_parts,lines.len());
    return count_valid_squares(&square_parts, squares) as u32
}


fn main() {
    let lines = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];
   println!("{}",count(lines));
}
