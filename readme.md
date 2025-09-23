 # When to choose what template

 - use ```cp.rs ``` when input test cases around <= 10e4 
 its basically BufRead + println! , reads line-by-line 

- use ```tmp2.rs1``` when input test cases constrain >= 10e5 or 10e6 , as bufreader works best for large inputs ..
this ver badsically uses Scanner + BufWriter,
reads all stdin once


