Integers are always unsigned.

Block metadata:
* element_length: 16 bit integer specifying size in bytes of each row
* num_blocks: 16 bit integer specifying number of blocks (max 65536 in file)
* block_info: a struct with the following fields, as a list of size `num_blocks` 
  * min_value: 64 bits to describe the minimum value found in this block
  * max_value: 64 bits to describe the maximum value found in this block
  * starting_offset: 64-bit integer describing the offset in the file
  * block_length: a 64-bit integer, the number of bytes in this block
  * row_id_start: a 64-bit integer, starting row number of this block
  * row_id_end: a 64-bit integer, ending row number of this block
  * rows: a 16 bit integer specifying the number of rows in the block

 