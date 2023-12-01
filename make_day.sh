#!/bin/bash

DayNum=$1

DIR_NAME="src/day_0${DayNum}"

mkdir ${DIR_NAME}
inputfile="${DIR_NAME}/day_0${DayNum}_input.txt"
touch ${inputfile}
touch "${DIR_NAME}/mod.rs"

PARTS=(1 2)

for i in ${!PARTS[@]}; do
    testfile="${DIR_NAME}/test_0${PARTS[$i]}.txt"
    touch ${testfile}
    rustfile="${DIR_NAME}/day_0${DayNum}_part_${PARTS[$i]}.rs"
    touch ${rustfile}
    echo "use crate::helper_functions::io::*;" >> ${rustfile}
    echo "" >> ${rustfile}
    echo "// Main function for day ${DayNum}" >> ${rustfile}
    echo "pub fn day_0${DayNum}_part_${PARTS[$i]}() {" >> ${rustfile}
    echo "  //filenames for input" >> ${rustfile}
    echo "  let filename = \"${inputfile}\";" >> ${rustfile}
    echo "  // let filename = \"${testfile}\"\n;" >> ${rustfile}
    echo "" >> ${rustfile}
    echo "  if let Ok(lines) = read_lines(filename) {" >> ${rustfile}
    echo "      // Consumes the iterator, returns an (Optional) String" >> ${rustfile}
    echo "      for line in lines {" >> ${rustfile}
    echo "          if let Ok(ip) = line {" >> ${rustfile}
    echo "              println!(\"{}\", ip);" >> ${rustfile}
    echo "          }" >> ${rustfile}
    echo "      }" >> ${rustfile}
    echo "  }" >> ${rustfile}
    echo "}" >> ${rustfile}
done

