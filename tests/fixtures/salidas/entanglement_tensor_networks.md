## Entanglement  and  tensor  networks  for  supervised  image  classification

John  Martyn , 1  Guifre  Vidal , 1  Chase  Roberts , 1  and  Stefan  Leichenauer 1

1 X,   The  Moonshot  Factory,  Mountain   View,   CA  94 04 3,   USA

Tensor  networks ,  originally  designed  to  address  computational  problems  in  quantum  many-body

physics ,  have  recently  been  applied  to  machine  learning  tasks .  However ,  compared  to  quantum

physics ,  where  the  reasons  for  the  success  of  tensor  network  approaches  over  the  last  30  years  is  well

understood ,  very  little  is  yet  known  about  why  these  techniques  work  for  machine  learning.  The  goal

of  this  paper  is  to  investigate  entanglement  properties  of  tensor  network  models  in  a  current  machine

learning  application ,  in  order  to  uncover  general  principles  that  may  guide  future  developments .  We

revisit  the  use  of  tensor  networks  for  supervised  image  classification  using  the  MNIST  data  set  of

handwritten  digits ,  as  pioneered  by  Stoudenmire  and  Schwab   [Adv .  in  Neur .  Inform .  Proc .  Sys .  29 ,

4799  ( 20 1 6) ] .  Firstly  we  hypothesize  about  which  state  the  tensor  network  might  be  learning  during

training .  For  that  purpose ,  we  propose  a  plausible  candidate  st ate  | Σ` i   (built  as  a  superposition

0 of  product  states  corresponding  to  images  in  the  training  set )  and  investigate  its  entanglement

2 properties .  We  conclude  that  | Σ` i   is  so  robustly  entangled  that  it  cannot  be  approximated  by

0 the  tensor  network  used  in  that  work,  which  must  therefore  be  representing  a  very  different  state .

2  Secondly,  we  use  tensor  networks  with  a  block  product  structure ,  in  which  entanglement  is  restricted

l within  small  blocks  of  n  ×   n  pixels/qubits .  We  find  that  these  states  are  extremely  expressive  (e . g.

u training  accuracy  of  99 . 97%  already  for  n  =  2 ) ,  suggesting  that  long-range  entanglement  may  not

# J

be  essential  for  image  classification .  However ,  in  our  current  implementation ,  optimization  leads  to

2 over-fitting,  resulting  in  test  accuracies  that  are  not  competitive  with  other  current  approaches .

# 1

# ]

I.  INTRODUCTION
 One  might  thus  expect  tensor  networks  to  work  well

# h

p in  machine  learning  due  to  their  expressive  power  and

- the  observation  that  patterns  in  real-world  data  are  rel

t Over  the  past  decade ,  research  in  artificial  intelligence
 –

n atively  simple   [2 5] .  In  current  studies   [4 1 2] ,  a  tensor

has  unveiled  a  symbiotic  relationship  between  physics

a network  architecture  is  selected ,  and  its  tensors  are  op

and  machine  learning.  For  instance ,  neural  networks

u  timized  so  as  to  minimize  a  loss  function  on  a  training

q have  been  used  to  locate  phase  transitions  in  spin  mod

[ set .  Subsequently,  its  performance  is  evaluated  on  the

els  and  even  develop  equations  of  motion  from  empirical

– test  sets .  These  methods  have  been  shown  to  work  sur

dat a   [ 1 3] .  On  the  flip  side ,  tensor  networks ,  initially

1 - prisingly  well ;  for  instance ,  the  MP S  model  can  achieve

devised  to  model  quantum  many body  states ,  have  been

v test  accuracies  upwards  of  99%  on  the  MNIST  data  set

successfully  applied  to  supervised  learning  t asks ,  such  as

2  of  handwritten  digits   [4] .

8 the  recognition  of  handwritten  digits ,  medical  image  clas

0 sification ,  and  anomaly  detection   [4–1 2] .
 In  quantum  physics ,  the  success  of  tensor  networks

6 Inspired  by  the  well-documented  success  of  tensor  net such  as  MP S  is  ultimately  based  on  a  well  understood

0. works  in  quantum  many-body  physics  over  the  last  30
 fact .  Namely,  tensor  networks  share  an  important  struc

7 years ,  these  machine  learning  studies   4–1 2  have  incorpo tural  property  with  the  quantum  states  (e . g.  ground

[ ] 

0 rated  networks  such  as  the  matrix  product  state   MP S 
 states  of  local  Hamiltonians)  that  they  try  to  approxi

( ) 

0 1 3–1 8 ,  the  tree  tensor  network   1 9 ,  20 ,  and  multiscale
 mate .  This  property  is  known  as  the  area  law  of  entan

2: e[ ntan ]lement  renormalization  ans[ atz   2]1   22 .  Introduc glement ,   [26 ,  2 7] .  How  about  in  machine  learning?  Sup

g [ , ]

v tions  to  tensor  networks  in  the  language  of  machine  learn pose  we  use  the  above  embedding  into  an  exponentially

i ing  can  be  found  in  Refs .   23 ,  24 .  It  is  import ant  to  keep
 large  vector  space ,  so  as  to  encode  the  dat a  into  a  quan

[ ]

X in  mind  that  tensor  network  models  are  linear  models
 tum  state  (see  Sec .  II  for  a  definition  of  quantum  states) .

# r

a with  an  input  space  that  is  exponentially  large  in  the
 What  property  do  typical  data  sets  have  that ,  upon  this

number  of  features   for  instance ,  the  number  of  pixels
 embedding  into  a  quantum  state ,  might  play  an  analo

in  an  ima e .  The  da( ta  is  first  embedded   non-linearl ! 
 gous  role  to  that  of  the  area  law  in  quantum  physics?  Al

g ) ( y )

in  this  exponentially  large  vector  space   see  Sec .  II  for  a
 though  a  direct  answer  seems  elusive ,  it  must  have  to  do

(

discussion  of  this  embedding .  Thanks  to  the  embedding ,
 with  correlations ,  e . g . ,  between  neighboring  pixels  in  an

linear  models  in  this  vector  s) pace  have  strong  expressive
 image .  After  embedding  a  set  of  images  in  an  exponen

ower .  However  the  de end  on  ex onentiall  man 
 tially  large  vector  space ,  these  correlations  are  formally

p , y p p y y

parameters –  that  is ,  they  are  afflicted  by  the  curse   of
 related  to  entanglement  in  quantum  physics .  The  goal

dimensionality.  The  magic  of  tensor  networks  is  that
 of  this  paper  is  to  explore  the  entanglement  properties

they  offer  a  manageable ,  efficient  description  of  a  re of  tensor  networks  when  used  for  machine  learning.  For

stricted  class  of  linear  models  in  this  high-dimensional
 concreteness ,  we  focus  on  supervised  image  classification

vector  space .  Linear  models  restricted  to  be  of  the  tensor
 of  the  MNIST  dataset  of  handwritten  digits ,  following

network  class  appear  to  still  ret ain  a  significant  amount
 Ref.   [4] ,  and  present  two  main  results .

of  their  expressive  power .
 The  first  result  refers  to  the  amount  of  entanglement

2

in  tensor  networks  for  machine  learning.  We  consider
 II.  PROTOCOL  FOR  SUPERVISED  IMAGE

an  embedding  of  the  MNIST  images,  which  are  com CLASSIFICATION  WITH  TENSOR  NETWORKS

prised  of  2 8  ×   2 8  pixels ,  in  a  st ate  of  a  square  latt ice

of  28  ×   28  qubits .  We  then  introduce  a  sum  st ate ,  | Σ` i ,
 In  this  section  we  discuss  the  methodology  of  applying

of  the  28  ×   28  qubits ,  built  as  a  linear  combination  of
 tensor  networks  to  supervised  learning ,  focusing  on  the

embedded  images .  (Here  `  is  a  class  label  that  will  be
 problem  of  image  classification .  We  first  summarize  the

described  later  on) .  We  initially  regarded  the  sum  st ate
 approach  laid  out  in  Ref.   [4] ,  after  which  we  discuss  our

| Σ` i   as  a  plausible  candidate  for  what  the  MP S  model  in
 modified  protocol .

Ref.   [4]  might  be  attempting  to  learn .  We  found ,  how

ever ,  that  the  sum  st ate  | Σ` i   has  very  large  amounts  of

entanglement ,  making  it  impossible  for  the  MP S  model
 A .  Previous  Work

to  learn  it ,  even  approximately.  We  thus  conclude  that

the  MP S  successfully  used  in  Ref.   [4]  for  image  classifi Previous  works   [4 ,  6–1 0]  that  perform  supervised  learn

cation  must  represent  some  very  different ,  less  entangled
 ing  with  tensor  networks  employ  the  following  protocol .

st ate  of  the  28  ×   28  qubits .
 For  concreteness ,  consider  supervised  learning  of  scale

gray  images ,  where  each  image  is  made  of  N  pixels .  For

instance ,  in  the  MNIST  data  set  of  handwritten  digits ,

each  image  is  made  of  N  =  28  ×   28  =  784  pixels .  The

The  above  result  referred  to  the  amount  of  ent angle dat a  of  an  image  is  stored  in  a  vector  x  ∈  V  ,  where  V  is  a

ment  in  a  particular  state .  Our  second  result  refers  in vector  space  of  dimension  N .  Each  component  xj  of  this

stead  to  the  range,  in  space ,  of  entanglement .  Entangle vector  corresponds  to  a  pixel ,  that  takes  the  normalized

ment  correlates  different  parts  of  the  system ,  and  we  may
 values  xj  ∈  [0 ,  1 ] .  Here  0  corresponds  to  a  white  pixel

ask  about  how  dist ant  these  parts  are .  For  this  purpose ,
 and  1  to  a  black  pixel .

we  divide  the  28  ×   28  qubits  pixels  into  blocks ,  indexed
 The  image  vector  x  ∈  V  is  then  mapped  to  a  vector

by  b ,  of  n  ×   n  adj acent  qubits ,  and  consid
 er  tensor  net | Φ (x ) i   in  a  2 N  -dimensional  vector  space  W ,

works  t hat  represent  a  st at e  | Ψ`B
 P S
 i   =
 N b
 | ψ
 
`b

 i   t hat  fac

b
 
 N

torizes  as  t he  pro duct  of  individual  st ates
  ψ
`
  for  each
 ∼

of  the  blocks .  By  construction ,  this  b lock  product  state
 W  =
 O Wj  ,  ( 1 )

(BP S )  wavefunction  | Ψ`B
 P S
 i   only  has  ent anglement  within
 j = 1

each  block  b .  That  is ,  | Ψ`B
 P S
 i   only  has  short  range   entan b  a  transformation  Φ  :  V  →  W  known  as  the  feature

y

glement.  Our  second  result  is  the  realization  that  this
 ma  Φ  :  x  7→   Φ x .  Above   W   is  a  2-dimensional

 p , | ( ) i , j

simple  tensor  network  with  only  short  range  entangle vector  space .  Following  the  language  of  quantum  infor

ment  within  each  block  is  already  extremely  expressive ,
 mation ,  we  refer  to  space  W   as  a  qubit ,  we  call  vectors

j

in  that  it  leads  to  very  high  accuracy  when  classifying  the
 such  as  Φ x   “wavefunctions”  or   “states”  and  we  re 

| ( ) i , p

training  set  even  for  small  blocks  made  of  2  ×   2  qubits .
 resent  them  with  kets    .  Accordin l ,  we  sa  that  the

 | i g y y

However ,  the  optimization  of  the  model  results  in  signif feature  map  Φ  maps  an  image  x  of  N  pixels  into  a  state

icant  over-fitting .  Indeed ,  the  trained  model  generalizes
 Φ x  ∈  W  of  N  ubits .  The  feature  ma  Φ  is  chosen

| ( ) i q p

poorly  to  the  test  set ,  for  which  the  accuracy  is  not  yet
 such  that  the  resulting  st ate  Φ x   is  normalized  to  1   in

| ( ) i (

competitive .  We  are  still  hopeful  that  by  training  the
 L
2  norm  i . e .  Φ x Φ x   =  1 .

) , h ( ) | ( ) i

model  with  a  different  optimization  algorithm ,  we  may
 The  feature  map  Φ  is  also  often  taken  to  be  comprised

obt ain  much  better  test  accuracies ,  although  we  leave
 of  local  feature  maps  φ
j
 ,  which  are  applied  to  entry  x   :

j

this  for  subsequent  explorations .

N

| Φ ( x ) i   =
 O | φ
j
 ( xj  ) i , 
  φ
j
 ( xj  ) 
  ∈   Wj  .   ( 2 )

j = 1

The  rest  of  the  paper  is  organized  as  follows .  In  Sec .

- That  is ,  each  pixel  is  mapped  into  a  qubit ,  and  the  result

II ,  we  summarize  the  general  set up  (embedding,  tensor

ing  st at e  is  called  a  p rodu c t  s tat
 e ,  since 
  it  ca
 n  b e  ex
 pressed

network ,  loss  funct ion ,  et c )  used  in  previous  st udies ,  and
  1
  2
 · · ·

- a
 s  a  t ensor  pro duct  | Φ ( x ) i   =
  φ
 ( x 1 ) 
  ⊗
  φ
 ( x 2 ) 
  ⊗        ⊗

then  describe  our  own  set up ,  which  differs  slightly  from

t hose  of  previous  st udies .  In  Sec .  III  we  intro duce  t he
  φ
N  ( x N  ) 
 .  A  typical  lo cal  feat ure  map  is

sum  st ate  | Σ` i   and  st udy  it s  ent anglement  prop ert ies ,  to
 π
 
 π

conclude  t hat  it  is  t o o  ent angled  t o  b e  learned  by  t he
 | φ
j
 ( xj  ) i   =  cos xj
 | 0 i   +  sin xj
 | 1 i ,  ( 3 )

 2
   2
 

MP S  used  in  Ref.   [4] .  In  Sec .  IV  we  introduce  the  block

pro duct  st ate  | Ψ`B
 P S
 i ,  which  we  realize  in  not  one  but  two
 where  { | 0 i ,  | 1 i }  is  an  orthonormal  basis ,  known  as  the

different  tensor  network  models  (dubbed  nearest  neighbor
 computational  basis  of  the  qubit .  Notice  that  this  feature

BP S ,  and  snake  BP S )  and  analyze  how  the  two  different
 map ,  which  acts  in  the  same  way  across  all  pixels  j  of  the

realizations  perform .  Finally,  in  Sec .  V  we  summarize
 image ,  maps  white  pixels   ( xj  =  0 )  to  the  | 0i   st ate  and

our  result s . 
 black  pixels   ( xj  =   1 )  t o  t he  | 1 i   st at e .

3

For  ease  of  not ation ,  in  the  rest  of  this  paper  we  write
 In  terms  of  expressive  power  and  comput ational  costs ,

| x i   to  mean  the  st ate  | Φ (x ) i .  After  the  feature  map  has
 the  product  st ate  is  the  least  expressive  and  least  expen

been  applied ,  images  are  classified  as  follows .  Let  { | T` i }
 sive ,  with  computational  memory  and  time  (per  sample)

denote  a  set  of  N- qubit  variat ional  st ates  enco ded  in  a
 scaling  as  O ( N ) .  In  contrast ,  a  generic  st ate  | Ψ
`g
en .
 i   is

tensor  network  model ,  where  st ate  | T` i  ∈  W ,  and  the
 the  most  expressive   (it  can  express  any  linear  map  in

g e n .

index  `  is  a  label  for  the  classes  under  consideration .  For
 W ! ) .  However ,  storing  and  using  a  generic  st ate  | Ψ
`
 i

instance ,  `  ∈  { 0 ,  1 ,  . . . ,  9 }  for  the  MNIST  data  set  of  hand incurs  computational  memory  and  time  that  grows  expo

written  digits .  Given  an  image  x  encoded  in  the  st ate  | x i ,
 nentially  in  N ,  and  it  is  thus  not  an  affordable  option  for

x  is  classified  as  the  label  k  for  which  the  overlap  | hTk | x i | 
 large  N .  Finally,  the  MP S  | Ψ`M
 P S
 i   sits  between  the  pre

is  largest :
 vious  two  options .  It  is  more  expressive  than  a  product

st ate  but  less  so  than  a  generic  st ate ,  and  it  has  compu

k  =  argmax`
 | hT` | x i | .  (4) 
 t at ional  cost  O ( χ
2 N )   (p er  sample ) .

One  thus  finds  a  trade-off  between  expressive  power

This  model  is  then  trained  by  choosing  the  varia and  computational  efficiency  depending  on  the  complex

tional  parameters  in  the  tensor  network  such  that  some
 ity  of  the  tensor  network .  While  a  tensor  network  model

loss  function  is  minimized  on  the  training  set  T  =
 needs  to  be  both  sufficiently  expressive  and  computation

{ ( x
(i )
 ,  y (i )
 ) }
iN=T1
 .  Here ,  x
(i )  are  t he  images  in  t he  t rain ally  efficient  for  a  given  t ask ,  generaliz at ion  is  yet  also

ing  set ,  and  y
 (i)  are  the  corresponding  correct  labels  for
 another  very  import ant  property  to  t ake  into  considera

these  images  (i . e .  the  train  labels) ,  whereas  NT  denotes
 tion .  Avoiding  over-fitting  in  order  to  achieve  sufficient

the  number  of  images  in  the  training  set .  Previous  stud generalization  relates  not  only  to  the  model ,  but  also  to

ies  employed  the  quadratic  loss  function
 how  it  is  optimized ,  making  a  systematic  analysis  much

more  difficult .

1 
 NT
 
 
 
 
 
 
 2

J
   { | T` i }  =
 X X  
 
 T`
  x
 ( i ) 
     −   δ` , y ( i ) 
 ,   ( 5 )

2
  

i= 1
 `
 B .  Modified  Approach

where  δ` , y ( i )  is  the  Kronecker  delt a .  This  loss  function
 

In  t his  work  we  employ  a  variat ion  of  t he  proto col  out

p enalizes  t he  difference  b etween
  
 
 T`
  x
(i )
  i 
    and  it s  ideal
 lined  ab ove .  We  use  t he  same  lo cal  feat ure  map  Φ  in

output ,  δ` , y ( i )  ( 1  for  the  correct  lab el ,  and  0  otherwise) .
 Eq.   ( 3 )  to  enco de  the  image  dat a  x  into  a  pro duct  st ate

Finally,  once  the  tensor  network  model  has  been
 | x i .  Given  a  tensor  network  model  that  produces  a  state

trained  using  the  training  set ,  it  is  tested  by  applying
 | T` i   for  each  class  ` ,  we  also  use  the  same  classification

the  feature  map  Φ  to  the  images  in  the  test  set  and  by
 criterion  in  Eq.   (4) .  However ,  we  use  a  different  loss

then  classifying  them  using  Eq.   (4) .
 function .

Given  the  feature  map  in  Eqs .   ( 2 ) - ( 3 )  and  the  loss  func Let
  x
(i)
 be  the  st ate  corresponding  to  embedding  the

tion  in  Eq.   ( 5 ) ,  the  performance  of  the  tensor  network
 training  image  x
(i)
 ,  and  let  us  first  define  a  probability

model  still  depends  critically  on  which  specific  tensor  net distribution   inspired  by  the  so-called  Born  rule  of  quan

(

work  we  use  in  order  to  encode  the  variational  st ates  | T` i .
 tum  mechanics   iven  b

) g y

Let  us  consider  three  examples :

x 
 ( i ) 
 T 2

•  P ro duct  st at e :  t he  simplest  p ossible  t ensor  network
 p
 y
 ( i )   =  `
 ≡
 |
 h |i 
 ` i | 2
 .   ( 6 )

P S
   P k
 | h x
 ( ) | Tk i |

mo del  corresp onds  t o  a
  pro
 duct  st at e ,  | T` i   =  | Ψ`
 i ,

where  | Ψ`P
 S
 i   =
 N N=
 1
  
ψ
`j

 sp ecifies  a  different  st ate
 Not ice  t hat ,  indeed ,  t his  is  a  probability  distribut ion

j  E since  by  const ruct ion  we  have

ψ
`j
 ∈  Wj  for  each  of  t he  N  qubit s .  S ince  t he  st ate

E

of  each  qubit  can  b e  sp ecified  wit h  2  paramet ers ,
 p
 y
 ( i )   =  `
 ≥   0 ,
 p
 y
 ( i )   =  `
 =   1 .   ( 7)

we  can  specify  the  product  st ate  | Ψ`P
 S
 i  ∈  W  using
   X`
  

2N  parameters .

Notice  also  that  we  can  replace  the  classification  criterion

•  Generic  st ate :  In  the  opposite  extreme ,  the  most
 (4)  with  the  equivalent  classification  criterion

complicated  tensor  network  model  would  be  to  not

restrict  t he  N- qubit  st ate  at  all ,  but  consider  in k  =  argmax p
 y
 (i )  =  `
 .  8

= ge n . 
 ge n . 
 `    ( )

st ead  a  generic  st at e ,   | T` i     | Ψ
`
 i ,  where  | Ψ
`
 i   ∈

W  is  specified  by  2 N  parameters .

Then ,  instead  of  optimizing  a  quadratic  loss ,  we  optimize

the  negative  log-likelihood :

•  Matrix  product  state  (MP S ) :  In  between ,  one  finds

t he  MP S ,  | T` i   =  | Ψ`M
 P S
 i ,  as  used  in  Ref.   [4] .  The
 NT

M P S 
 2  
 
 
 
 
 i

MP S  | Ψ`
 i  ∈  W  is  sp ecified  by  O ( χ
 N )  parame J
 { | T` i } =  −
 δ` ( i )  log  p ( y
 ( ) =  `) 
 .  ( 9 )

   X X ,y   

t ers ,  if  it  is  const ruct ed  from  χ  ×   χ  mat rices . 
 i = 1
 `

4

This  loss  function  is  minimized  when  it  perfectly  classi Here  { λα }
χα
= 1  are  the  (non-vanishing)  Schmidt  coeffi

fies  the  training  set ,  namely  when  we  have  p (y
 (i)  =  `)  =
 cients ,  which  are  sorted  in  dˆecrˆeasing  or
der ,  namely

δ` , y ( i )  .  Not ice  t hat  our  loss  is  similar  t o  t he  loss  func λα  ≥   λα + 1  ≥   0 ,  and
  fulfill  h Σ
 ` | Σ
 ` i   =
 P α
 ( λα ) 
2  =   1 .

t ions  used  in  Refs .   [7 ,  9] .  However ,  here  we  work  wit h
 In  t urn ,  t he  st ates  {
  ϕ
Aα

  }  form  an  ort honormal  basis ,

t he  logarit hm  of  t he  overlap ,  inst ead  of  t he   ( logarit hm  of
 ϕ
Aα
 
  ϕ
Aα 0
 
 =  δα α 0  ,  and  t he  same  applies  t o  {
  ϕ
αB
 
 } ,  wit h



t he  exp onent ial  of  t he )  overlap .  O ur  formulat ion  is  b et 
B
  
B0
 
 =  δ 0  .

- 
 ϕα
  ϕα  αα

ter  prepared  to  deal  with  overlaps  in  an  N qubit  Hilbert
 In  order  to  characterize  the  entan lement  in  the  above

g

space ,  which  are  exponentially  large   (or  small)  in  N .
 st ate ,  we  consider  two  quantities .  The  first  one  is  the

Finally,  another  important  difference  is  that  instead  of
 entan lement  entro   S A   e uivalentl   S B  of  the

MP S
 g ˆ py ( ) ( q y, ( ) )

using  a  MP S  | Ψ`
 i   as  in  Ref.   [4] ,  here  we  will  explore

st ate  | Σ` i   with  respect  to  the  partition  A : B ,  which  is

the  use  of  a  simpler  tensor  network ,  representing  a  block

B P S
 defined  as

pro duct  st ate  | Ψ`
 i ,  as  describ ed  in  Sect .   IV .

χ

− 2 
 
 2

S ( A )   ≡   X ( λα ) 
 log    ( λα ) 
  ,  ( 1 3 )

III.  ENTANGLEMENT  AND  IMAGE
 α=1

CLASSIFICATION

and  it  is  a  measure  of  how  much  correlation  there  is

 between  parts  A  and  B .  For  our  purposes ,  the  entan

In  this  section  we  investigate  the  entanglement  struc

glement  entropy  provides  a  useful  lower  bound ,  namely

ture  of  a  st ate  | Σ` i  ∈  W  that  we  initially  thought  might
 S A

e
 ( ) ,  on  the  minimal  bond  dimension  that  needs  to  be

be  closely  related  to  what  a  tensor  network  model  might

connectingˆ  parts  A  and  B  in  a  tensor  network  represen

be  trying  to  learn .  We  will  conclude ,  however ,  that  | Σ` i

t at ion  of  | Σ
 ` i ,  see  F ig .   1 .

is  too  ent angled  for  the  tensor  network  model  in  Ref.   [4]

to  learn  it ,  even  approximately.

Specifically,  for  each  label  `  we  consider  the  st ate

| Σ ` i   ≡    

x
 ( i ) 
 ,   ( 1 0 )

X  E

i ;   y ( i ) = `

t hat  is ,  a  linear  combinat ion  of  all  t he  st at es
  x
( i )
  cor

resp onding  to  images  x
(i )
 in  t he  training  set  t hat  are
 FIG .  1 .  A  partition  of  a  system  into  regions  A  and  B .  In

 order  to  represent  this  system  by  a  tensor  network ,  the  bond

classified  in  class  ` .  Note  that  this  st ate  is  not  normal S ( A)

dimension  in  between  regions  A  and  B  must  be  χ  ≈  e
 .

iz ed .

By  construction ,  this  state  has  significant  overlap  with

any  image  in  the  training  set  that  is  labelled  ` .  Indeed ,
 A  more  direct  measure  of  the  required  bond  dimension

for  such  images  hx
(i)
 | Σ` i  ≥  1 .  Hence ,  using  | Σ` i   for
 is  given  by  a  second  quant ity,  the  S chmidt  rank  χ ,  that

classification  yields  reasonable  accuracies  on  the  training
 is ,  the  number  of  non-vanishing  Schmidt  terms  in  the

set ;  it  was  also  observed  to  produce  reasonable  accuracies
 decomposition  ( 1 3) .  When  all  the  Schmidt  coefficients

on  the  test  set .  May  it  then  be  the  case  that  the  MP S
 are  of  similar  size ,  then  the  Schmidt  rank  χ  is  a  robust

model  | Ψ`M
 PS
 i   in  Ref.   [4]  somehow  approaches  | Σ` i   during
 measure  of  the  bond  dimension  needed  in  a ˆ tensor  net

training?  To  address  this  question ,  next  we  study  the
 work  that  accurately  approximates  the  st ate  | Σ
 ` i ,  and  we

entanglement  structure  of  | Σ` i ,  and  we  compare  it  to  the
 have  χ  ≈  e
S (A)
 .  However ,  if  the  Schmidt  coefficients  have

entanglement  structure  allowed  in  an  MP S .
 very  different  sizes ,  then  it  might  be  possible  to  truncate

(ignore)  some  of  the  terms  in  the  Schmidt  decomposi

tion  corresponding  to  the  smallest  Schmidt  coefficients

A .  Schmidt  rank  and  entanglement  entropy
 while  sˆtill  obtaining  an  accurate  approximation  of  the

st ate  | Σ
 ` i ,  in  which  case  a  tot al  bond  dimension  smaller

Let  us  partition  the  N  =  28 × 28  qubits  into  two  sets  A
 than  χ  may  already  be  sufficiˆent  in  an  approximate  tensor

and  B ,  where  A  will  be  some  subset  of  adj acent  qubits  to
 netwoˆ rk  represent ation  of  | Σ
 ` i .  Below  we  report  results

bˆe  described  below .  Let  us  define  a  normalized  version
 for  | Σ
 `=3 i ,  that  is ,  for  MNIST  images  of  the  digit  ‘ 3 ’ ,

| Σ
 ` i   of  st ate  | Σ` i ,  that  is
 although  the  same  construct ion  for  other  values  of  the

class  lab el  `  ∈  { 0 ,  1 ,   . . . ,  9 }  pro duces  very  similar  result s .

ˆ 
 | Σ ` i

| Σ ` i   ≡   
 ,   ( 1 1 )

ph Σ` | Σ` i

B .  Part it ion  into  top  and  bottom  halves

and  then  expand  it  in  its  Schmidt  decomposition

χ
 In  Ref.   [4] ,  the  MP S  snakes  around  the  28 × 28  square

ˆ

Σ
 `   =
 λ α
  
A
  
B
 .   1 2 
 latt ice  of  qubit s   (which  had  b een  reduced  t o  a   1 4  ×   1 4

| i X  ϕα
   ϕα
  ( )

α= 1
 square  lattice  of  qubits  for  simplicity)  by  moving  from  left

5

to  right ,  then  right  to  left ,  and  so  on ,  while  descending
 (even  approximately)  would  need  to  have  bond  dimen

through  the  grid ,  see  Fig .  2 .  That  means  that  the  top
 sion  ≈  NΣ .

half  A  and  bottom  half  B  of  the  lattice ,  each  made  of
 In  the  case  of  a  flat  spectrum  λα  ≈  1 /
√NΣ ,  the  ent an

1 4  ×   28  =  392  qubits ,  are  only  connected  by  one  single
 glement  entropy  is  given  by  S (A)   ≈  log  NΣ .  Since  the

bond  index .  In  Ref.   [4] ,  this  bond  index  was  chosen  to
 spectrum  in  Fig .   ( 3 )  is  very  flat ,  here  we  do  not  learn

take  up  to  1 20  values ,  in  which  case  the  classification  task
 anything  new  by  studying  at  the  entanglement  entropy

had  test  accuracy  of  99 . 03 % .
 (not  plotted) ,  but  since  this  is  the  most  popular  measure

of  ent anglement ,  we  include  reference  to  it  to  facilit ate

comparison  with  other  research .

Finally,  we  poˆint  out  that  computing  the  Schmidt  de

composition  of  | Σ
 ` i   in  vector  spaces  of  very  large  dimen

sion   (notice  that  2 8  ×   2 8  =  784  qubits  are  described  by

a  vector  space  W  of  dimension  2 784  ≈  1 0236 )  can  be  ac

complished  with  computational  cost  O ( (NΣ ) 
3
 )  using  the

strategy  described  in  the  Appendix.

FIG .  2 .  Example  of  an  MP S  that  snakes  around  a  two

dimensional  square  lattice  of  qubits ,  used  to  encode  images  of

8 × 8  pixels .  The  discontinuous  line  partitions  the  top  and  bot

tom  halves  of  the  image .  The  MP S  only  has  one  bond  index,

emphasized  with  an  arrow,  connecting  the  top  and  bottom

halves .

Fig .  3  shows  the  Schmidt  spectrum  of  this  partition ,

as  a  function  of  the  tot al  number  NΣ  of  images  used  in
 FIG .  3 .  Schmidt  spectrum  for  different  NΣ  in  the  range

the  training  set ,  for  | Σˆ 
 3 i   –  that  is ,  for  images  correspond 1 0 − 1 280  of  the  state  | Σ3 i ,  constructed  from  encoded  MNIST

ing  to  the  digit  3 .  We  find  that  the  S chmidt  sp ectrum  is
 images  of  the  digit  ‘ 3 ’ .  Part  A  is  the  top  half  of  the  square

 latt ice  of  qubit s .   ( Not ice  t hat  we  plot   ( λα ) 
2
 instead  of  λα ) .

essentially  flat ,  indicating  that  the  requirˆed  bond  dimen

For  small  NΣ  t he  lines  are  horizont al ,  t hat  is ,  all  t he  S
chmidt

sion  for  an  accurate  MP S  description  of  | Σ 3 i   is  essentially

values  have  essentially  the  same  magnitude  λα  ≈  1 /
√NΣ .

equal  to  NΣ .  For  inst ance ,  for  NΣ  =  1 280  images ,  the

maximal  bond  dimension  1 20  used  in  Ref.   [4]  results  in

an  MP S  that  caˆnnot  be ,  even  by  far ,  an  accurate  ap

proximation  to  | Σ
 3 i ,  because  1 20    1 280 .  We  conclude
 C .  Cent ral  block  of  size  L  ×   L

that  the  MP S  in  Ref.   [4] ,  which  successfully  classifies

thˆ e  images ,  is  not  representing  a  state  anywhere  close  to
 For  completeness ,  we  have  also  explored  the  amount  of

| Σ
 3 i .
 ˆ ent anglement  entrˆopy  of  a  square  region  A  of  size  L  ×   L .

A  flat  spectrum  of  Schmidt  values  in  | Σ
 3 i   indicates
 Specifically,  for  | Σ
 3 i   we  computed  the  average  entropy

that  the  bottom   ( and  top)  of  the  NΣ  images  in  the  train of  a  square  of  L  ×   L  qubits  in  a  central  window  of  size

ing  set  are  encoded  in  essentially  orthonormal  st ates .
 1 0  ×   1 0 .  For  inst ance ,  when  L  =  1 ,  we  looked  at  the

That  follows  simply  from  the  fact  that  any  two  images
 average  entropy  of  all  1 00  qubits  in  this  central  window ;

typically  differ  in  a  few  number  of  pixels  both  on  the
 when  L  =  2 ,  we  looked  at  the  average  entropy  of  all  8 1

top  half  and  on  the  bottom  half.  For  larger  values  of
 2  ×   2  squares  of  qubits  in  this  window ;  and  so  on .

NΣ  we  see  that  the  Schmidt  values  are  no  longer  the
 ˆ We  display  our  results  in  Figure  4  below  for  st ates

same ,  although  they  are  still  very  similar .  This  indicates
 | Σ
 3 i   built  as  a  superposition  of  NΣ  images ,  for  a  range  of

that  some  of  the  images  in  the  training  set  are  now  a
 values  of  NΣ .  We  see  that  for  a  block  of  size  L  ×   L ,  the

bit  similar ,  in  that  their  overlaps  in  the  top  or  bottom
 entropy  appears  to  grow   (slighly  faster  than)  linearly  in

halves  are  no  longˆer  negligible .  However ,  an  accurate  ap the  perimeter  size  4L ,  before  saturating  very  close  to  its

proximation  to  | Σ
 3 i   still  requires  keeping  about  NΣ  of
 maximal  possible  value  for  NΣ  images ,  namely  log ( NΣ ) .

ˆ

the  Schmidt  values ,  so  that  an  MP S  representing  | Σ
 3 i 
 To  gain  further  insight ,  Figure  5  shows  the  Schmidt

6

indicates  that  one  could  in  principle  truncate  away  the

terms  in  the  Schmidt  decomposition  corresponding  to  the

smallest  Schmidt ˆ values  while  retaining  an  accurate  ap

proximation  to  | Σ
 3 i .  However ,  the  number  of  Schmidt

values  one  needs  to  keep  is  seen  to  grow  sharply  with

L ,  as  indicated  by  the  entanglement  entropy  in  Figure

4 .  This  implies  that  a  tensor  network  such  as  MP S  or

tree  tensor  network  wouˆ ld  require  a  very  large  bond  di

mension  to  represent  | Σ
 3 i ,  making  such  representation

inefficient .

IV.  EXPRESSIVE  POWER  OF  BLOCK

PRODUCT  STATES

In  the  previous  section  we  have  seen  that  the  st ate

| Σ` i   in  Eq.   ( 1 0 ) ,  built  by  simply  superposing  the  en

coded  images  of  class  `  in  the  training  set ,  was  very  ro

bustly  entangled ,  so  much  so  that  it  precluded  an  efficient

FIG .  4 .  Average  entanglement  entropy  S (A)  vs .  perimeter

4L  for  regions  A  consisting  of  L  ×   L  squares  in  a  1 0  ×   1 0
 represent at ion  in  terms  of  the  MP S  used  in  Ref.   [4]  to

ˆ

central  window ,  for  the  state  | Σ  3 i   constructed  from  encoded
 successfully  classify  this  dat a  set .  We  concluded  that  a

MNIST  images  of  the  digit  ‘ 3 ’ .
 tensor  network  such  as  an  MP S  does  not  need  to  be  able

to  represent  the  st ate  | Σ` i   in  order  to  b e  a  successful

model  for  image  classification .

With  this  insight ,  we  next  explore  the  use  of  other

simple  tensor  network  models  for  the  same  task.  Specif

ically,  we  will  consider  tensor  networks  that  represent

states  with  entanglement  restricted  within  small  blocks

of  qubits .  We  will  learn  that  these  simple  tensor  networks

are  already  very  expressive .  However ,  we  will  also  see

that ,  at  least  with  our  current  optimization  algorithm ,

these  models  suffer  from  over-fitting  and  therefore  gen

eralize  poorly  from  the  training  dat a  set  to  the  test  dat a

set .  We  will  then  investigate  ways  to  alleviate  this  prob

lem ,  with  partial  success ,  and  will  conclude  that  further

research  is  still  needed  to  prevent  over-fitting  in  these

otherwise  quite  promising,  surprisingly  simple  tensor  net

work  models .

A .  B lock  Product  States

FIG .  5 .  Schmidt  spectrum  { λα }  for  different  values  of  NΣ ,

wˆhen  region  A  is  a  3  ×   3  square  in  the  central  window  of

| Σ  3 i ,  constructed  from  encoded  MNIST  images  of  the  digit
 We  first  define  the  general  structure  of  the  st ates  used

‘ 3 ’ .   ( Notice  that  we  plot   ( λα ) 
2
 instead  of  λα . ) 
 in  t he  following  mo dels .  Given  t he  square  latt ice  of

28  ×   28  qubits  in  which  the  MNIST  images  have  been

encoded ,  we  consider  subdivisions  into  square  blocks  of

n  ×   n  adj acent  qubit s  for  n  =   1 ,  2 ,  3 ,  4 ,  see  Fig .   6  for  an

spectrum  in  the  case  where  part  A  is  a  square  block  of
 = = 

illustrat ion  wit h  n   3 .  For  n    1 ,  2 ,  3  and  4 ,  we  resp ec

3  ×   3  qubit s ,  again  as  a  funct ion  of  NΣ .  Not ice  t hat  t he
 2
 2
 2  2
 =

9  t ively  obt ain  2 8 ,   1 4 ,  9 and  7 such  blo cks ;  for  n   3 ,

vector  space  of  3  ×   3  =  9  qubits  has  dimension  2 =  5 1 2 ,

we  ignored  the  last  row  and  column  of  pixels   (nearly  all

wˆhich  provides  an  upper  bound  for  the  Schmidt  rank  of
 

of  which  are  black  anyway)  so  that  the  images  were  en

| Σ 3 i   with  respect  to  this  partition .  When  NΣ  =  1 0 ,  we

coded  in  a  square  lattice  made  of  2 7 × 2 7  qubits .  We  then

observe  a  rather  flat  Schmidt  spectrum ,  indicating  that
 “

t ake  the  tensor  network  st ate  | T` i   to  be  a   block  product

the  NΣ  images  are  embedded  in  fairly  orthogonal  states
 ” BPS

 st at e   | Ψ`
 i ,  namely  a  st
 at e 
  t hat  can  b e  writt en  as  t he

both  in  A  and  its  complement  B .  However ,  as  the  num  b

 tensor  pro duct  of  st ates
  ψ
`
  for  each  square  blo ck  b  of

ber  NΣ  of  images  grows ,  the  corresponding  states  in  re

- n  ×   n  qubit s ,  t hat  is

gion  A  st art  to  overlap  non trivially,  and  this  result s  in

a  sharply  decaying  sp ectrum  of  S chmidt  values ,  whose
 | Ψ
`B
 P S
 i  ≡  | ψ
`b

 i ,  ( 1 4 )

− 1
 − 9
 O

magnitude  is  seen  to  range  e . g .  from  1 0 to  1 0 .  This
 b ∈ Bn

A  block  product  state  is  represented  diagrammatically

in  Fig .   7 .  Not ice  t hat  | ψb i   is  it self  a  st at e  of  n
2  qubit s .

= n
2

Its  number  d   2 of  components  grows  very  fast  with

n .  Indeed ,  for  n  =   1 ,  2 ,  3 ,  and  4  it  is  d  =  2 ,  1 6 ,  5 1 2  and

65 , 536 ,  respectively.  We  will  then  further  specialize  the

block  product  state  structure ,  by  replacing  each  generic

= n
2
 

state  | ψb i   made  of  d   2 components  with  a  more  ef

ficient  tensor  network  representation .  Below  we  consider

two  options :  the  nearest  neighbor  block  product  state ,

which  consists  of  a  proj ected-entangled  pair  state  PEP S

[28]  within  each  n  ×   n  block ,  and  the  snake  block  prod

uct  st ate ,  which  is  an  MP S  within  each  n  ×   n  block ,  as

described  below .

7

FIG .  8 .  A  nearest  neighbor  block  product  state  (NNBP S ) .

+  α
 P`
 |   log ( Z` ) | ,  where  Z`  =  hT` | T` i .  In  our  analyses ,

we  let  α  ∼  O ( 1 ) .  We  display  results  below  in  Tables  I

and  I I .

Block  Size  Training  Accuracy  Test  Accuracy

1  ×   1  93 . 0 70 %  9 1 . 1 00 %

2  ×   2  99 . 96 7%  94 . 690 %

3  ×   3  99 . 92 5 %  95 . 470 %

FI G .  6 .  Tiling  a  grid  into  blo cks  of  size  n  ×   n ,  where  n  =  3 .
 4  ×   4  99 . 9 77%  9 5 . 42 0 %

TABLE  I .  Nearest  neighbor  block  product  state  applied  to

MNIST  dataset  of  handwritten  digits

Block  Size  Training  Accuracy  Test  Accuracy

1  ×   1  88 . 1 3 2 %  84 . 2 30 %

2  ×   2  92 . 788 %  86 . 540 %

3  ×   3  94 . 2 75 %  86 . 890 %

4  ×   4  94 . 940%  8 7 . 320%

TABLE  II .  Nearest  neighbor  block  product  state  applied  to

Fashion-MNIST  dataset

On  the  MNIST  dataset  of  handwritten  digits ,  we  see

FIG .  7.  Construction  of  a  block  product  state  from  the  set  of
 that  even  small  2  ×   2  blocks  can  achieve  nearly  1 00 %

blo cks .  Here ,  n  =  3 .

training  accuracy.  We  find  that  rather  remarkable .  It

means  that  such  a  simple  tensor  network  model  already

has  the  potential  of  being  able  to  classify  also  the  MNIST

images  in  the  test  set  with  the  same  accuracy   ( after  all ,

B .  Nearest  Neighbor  B lock  Product  State
 this  is  what  would  happen  if  we  included  the  test  set  in

the  training  set ) .  As  it  is  well-known ,  however ,  having

Fig .  8  depicts  a  nearest  neig
 hbo
 r   b lock  product  state
 enough  expressive  power  to  classify  all  the  images  is  only

( NNB P S ) ,  in  which  the  st ate
  ψ
`b

  for  blo ck  b  ∈  Bn  is
 useful  if  we  also  know  how  to  train  the  mo del ,  using  only

represented  by  a  PEP S ,  where  each  PEP S  tensor  has
 the  training  set ,  in  a  way  that  it  suit ably  generalizes  to

bond  indices  connecting  it  to  its  nearest  neigbor  tensors
 the  test  set .  And  this  is  where  our  approach  still  fails .

within  the  n  ×   n  block .  We  choose  the  bond  dimension
 For  a  2 × 2  block ,  our  current  optimization  scheme  results

χ  =  2 ,  so  that  a  PEP S  tensor  with  4  bond  indices  and
 in  poor  test  accuracies ,  under  9 5 % .  Blocks  of  size  3  ×   3

one  pixed  index  consists  of  2 5  =  32  parameters .  Notice
 and  4  ×   4  are  seen  to  again  lead  to  nearly  1 00 %  train

that  we  also  endow  each  tensor  with  a  class  label  ` .
 accuracies  but  much  lower  test  accuracies  under  96 % .

To  train  the  model ,  we  minimize  the  loss  function  out We  have  also  explored  performance  on  the  Fashion

lined  in  Sec .  II  B  with  the  Adam  optimization  algorithm .
 MNIST  dataset .  We  found  that  train  and  test  accuracies

In  addition ,  we  include  in  the  loss  function  a  regular monotonically  increase  with  block  size ,  but  again  the  test

ization  term  to  keep  the  normalization  of  { | T` i }  finite :
 accuracy  lags  behind  the  training  accuracy  significantly.

8

In  addition ,  as  this  data  set  is  more  complex  than  MNIST
 normalization  problems .  We  choose  α  ∼  O ( 1 )  and  vary

digits ,  we  do  not  achieve  1 00%  training  accuracy,  while
 the  bond  dimension  of  the  network  between  χ  =  2  and

the  test  accuracy  saturates  around  ∼  87% .  We  note  nev χ  =  1 8 .  We  display  results  below  in  Table  III .

ertheless  that  this  accuracy  is  comparable  to  that  of  Ref.

[9] ,  where  88%  test  accuracy  was  achieved  on  the  Fashion Block  Size  Bond  Dim.  Training  Accuracy  Test  Accuracy

MNIST  dat a  set  using  an  MP S  model .
 2  ×   2  χ  =  2  96 . 000%  94 . 700%

We  conclude  that  this  first  block  product  st ate  model
 2  ×   2  χ  =  3  97. 048%  95 . 330%

is ,  surprisingly,  expressive  enough  to  fit  the  training  set
 2  ×   2  χ  =  4  97 . 983%  95 . 7 1 0%

very  well ,  but  clearly  over-fits  the  dat a.

Block  Size  Bond  Dim.  Training  Accuracy  Test  Accuracy

3  ×   3  χ  =  2  94 . 5 98 %  94 . 000 %

C .  S nake  B lock  P roduct  State
 3  ×   3  χ  =  3  96 . 75 7%  95 . 1 30%

3  ×   3  χ  =  4  9 7 . 6 5 7%  9 5 . 890 %

In  an  attempt  to  reduce  over-fitting ,  we  have  explored
 3  ×   3  χ  =  6  97. 808%  95 . 640%

the  use
  of
  alternat ive  tensor  networks  to  represent  the
 3  ×   3  χ  =  1 2  98 . 36 7%  95 . 430%

st ate
  ψ
`b

 wit hin  each  blo ck .  Here  we  rep ort  on  one
 3  ×   3  χ  =  1 8  98 . 085 %  95 . 470 %

of  them ,  which  for  a  blo ck  of  size  4  ×   4  resulted  in

lower  training  accuracy  but  higher  test  accuracy  than
 Block  Size  Bond  Dim.  Training  Accuracy  Test  Accuracy

the  NNBP S  described  above .
 4  ×   4  χ  =  2  94 . 003%  93 . 870%

Fig .   9  depict s 
  a  sn
 ake   b lock  produ ct  state  ( S B P S ) ,  in
 4  ×   4  χ  =  3  96 . 342 %  9 5 . 300 %

which  t he  st at e
  ψ
`b

  for  blo ck  b  ∈  Bn  is  represent ed  by
 4  ×   4  χ  =  4  96 . 8 20 %  9 5 . 390 %

an  MP S  with  its  bond  index  scanning  the  n  ×   n  block  by
 4  ×   4  χ  =  6  97. 878%  96 . 200%

moving  from  left  to  right  in  the  top  row ,  then  right  to
 4  ×   4  χ  =  1 2  97 . 050%  95 . 340%

left  in  the  next  row ,  etc ,  imit ating  a  snake .  We  consider
 4  ×   4  χ  =  1 8  97 . 332 %  94 . 5 50%

blo cks  of  size  n  ×   n  for  n  =  2 ,  3 ,  4   ( not ice  t hat  t he  case

n  =  1  would  be  identical  to  the  previous  analysis) .  In  ad TABLE  III .  Block  product  state  constructed  from  MP S  ap

dition ,  in  order  to  reduce  variational  parameters  and/or
 plied  to  the  MNIST  dataset  of  handwritten  digits

frustrate  their  optimization ,  we  only  have  one  `  label  for

each  MP S ,  which  hangs  from  an  additional  tensor  con

nected  to  the  MP S  tensors  through  two  bond  indices ,  see

Fig .   ( 9 ) .  Using  a  single  class  lab el  `  for  the  whole  MP S

( as  opposed  to  having  a  class  label  `  on  each  tensor  of

the  MP S )  seems  to  help  lower  the  training  accuracy  while

lifting  the  test  accuracy.  This  may  be  due  to  the  fact  that

the  parameters  in  the  rest  of  the  MP S  tensors  are  shared

among  the  different  classes .  (We  also  implemented  the

same  ‘ single  class  label ’  on  each  PEP S  of  the  NNBP S  de

scribed  above ,  but  in  that  case  we  did  not  obt ain  better

result s . )

FIG .  9 .  A  snake  block  product  st ate   ( S BP S ) .

FIG .  1 0 .  Training  and  test  accuracies  of  snake  block  product

states  ( SBP S )  applied  to  the  MNIST  dataset  of  handwrit

ten  digits .  The  arrows  point  at  the  maximal  test  accuracy

obt ained  for  each  size  n  ×   n  of  t he  blo cks ,  for  n  =  2 ,  3 ,  4 .

Suggestively,  the  maximal  test  accuracy  is  seen  to  increase

monotonically  with  n .

From  this  data,  we  see  that  the  gap  between  train

We  train  the  SBP S  model  by  again  minimizing  the  loss
 ing  accuracy  and  test  accuracy  has  closed  significantly

function  outlined  in  Sec .  II  B  with  Adam  optimization
 compared  to  the  NNBP S  model  analysed  above .  This

and  add  the  same  regularization  term  before  to  prevent
 is  due  in  part  to  a  decrease  in  training  accuracy,  but

9

also  to  an  increase  in  test  accuracy.  More  specifically,
 6 , 000  images  in  MNIST ,  a  number  much  greater  than

starting  with  bond  dimension  χ  =  2  both  the  train  and
 the  largest  MP S  bond  dimension  χ  =  1 20  considered  in

test  accuracy  increase  for  small  but  increasing  values  of
 Ref.   [4] .  We  conclude  that  the  tensor  network  model

χ .  However ,  as  the  bond  dimension  grows  further ,  the
 must  be  learning  a  state  that  is  very  different  from  the

training  accuracy  generally  continues  to  grow ,  while  the
 sum  st ate  | Σ` i .

test  accuracy  reaches  a  peak  and  then  st arts  to  decrease ,
 In  our  second  investigation ,  we  defined 
 bloc
 k  product

signaling  again  over-fitt ing .  Overall ,  however ,  S B P S  is
 st ates  | Ψ`B
 P S
 i   t hat  factorize  into  st ates
  ψ
`b

  of  blo cks

seen  to  perform  better  than  NNBP S ,  in  the  sense  that  it
 b  made  of  n  ×   n  qubits .  By  construction ,  these  st ates

generalizes  better  and  achieves  greater  test  accuracy.
 only  contain  short-ranged  entanglement –   entanglement

We  also  report  that  using  a  redundant  parameteriza within  each  n  ×   n  block  of  qubits .  We  then  noticed  that

tion  of  the  MP S  tensor   (e . g .  a  bond  dimension  larger
 even  n  =  2  leads  to  very  large  training  accuracy,  close

than  needed  near  the  boundary  of  the  MP S ,  such  as  a
 to  1 00% ,  but  that  the  models  suffered  from  over-fitting,

value  larger  than  2  for  the  bond  dimension  of  the  first
 leading  to  poor  test  accuracy.  We  managed  to  partially

or  last  MP S  tensor)  results ,  surprisingly,  in  an  improved
 alleviate  over-fitting  and  improve  generalization  by  con

performance .  We  interpret  this  counter-intuitive  result
 sidering  different  tensor  network  representations  within

as  indicating  that  there  is  clear  room  for  improving  test
 each  block .  However ,  further  work  is  still  needed  before

accuracies  using  alternative  optimization  schemes .
 these  very  simple ,  yet  surprisingly  expressive  states  are

turned  into  competitive  models  for  supervised  image  clas

sification .  We  could  not  carry  such  investigation  here  due

to  time  constraints ,  but  we  hope  that  our  partial  findings

V.  DISCUSSION

are  already  useful  to  other  researchers  in  the  field .

Ent anglement  plays  a  clear-cut  role  in  the  use  of  tensor

In  this  work,  we  have  conducted  two  different  investi networks  for  quantum  many-body  systems ,  where  ground

gations  that  aimed  to  shed  light  into  the  role  of  ent angle st ates  of  local  Hamiltonians  obey  the  so-called  area  law  of

ment  in  supervised  image  classification  with  tensor  net entanglement  entropy,  that  tensor  networks  can  match .

works .  In  these  approaches ,  each  image  is  encoded  as  a
 In  contrast ,  much  less  is  known  about  the  role  that  en

vector  in  a  vector  space  whose  dimension  is  exponentially
 tanglement  plays  in  tensor  networks  for  machine  learn

large  in  the  number  of  pixels  in  an  image .  Then  a  tensor
 ing .  However ,  in  this  work  we  have  learned  that ,  despite

network  is  used  to  define  a  linear  model  in  this  massively
 of  the  fact  that  ent anglement  is  clearly  useful –   notice

large  vector  space ,  with  a  number  of  parameters  that  is
 that  the  training  accuracy  increased  significantly  in  our

only   (roughly)  proportional  to  the  number  of  pixels  in  an
 block  product  states  in  going  from  n  =  1   (unentangled

image .
 st ate)  to  n  =  2   ( st ate  ent angled  within  blo cks  of  2  ×   2

In  the  first  investigation ,  we  defined  a  sum  state  | Σ` i 
 qubits) –   large  amounts  of  entanglement  and  long  range

as  a  superposition  of  all  encoded  images  of  class  `  in
 may  not  be  needed  at  all .

the  training  set .  We  had  imagined ,  incorrectly,  that  this
 A cknow ledgements :  J . M .  and  G . V .  thank  Cutter

state  might  be  the  one  learned  by  e . g.  the  MP S  in  Ref.
 Coryell ,  Carlos  Fuertes ,  Anna  Golubeva,  and  Guy  Gur

[4] .  However ,  we  found  that  the  sum  st ate  is  massively
 Ari  for  advice  and  thoughtful  discussion .

entangled .  Approximating  it  by  an  MP S  would  require
 X ,  formerly  known  as  Google [x] ,  is  part  of  the  Alpha

the  bond  dimension  χ  to  be  roughly  equal  to  the  number
 bet  family  of  companies ,  which  includes  Google ,  Verily,

of  images  of  class  `  in  the  training  set ,  which  is  about
 Waymo ,  and  others  (www .x. company) .

[ 1 ]   L .  Wang ,  P hys .  Rev .  B  94 ,   1 9 5 1 0 5   ( 2 0 1 6 ) ,
 [8]   M .  Trent i ,  L .  Sest ini ,  A .  Gianelle ,  D .  Zuliani ,  T .  Felser ,

arXiv : 1 606 . 003 1 8   [cond-mat . stat-mech] .
 D .  Lucchesi ,  and  S .  Montangero ,  arXiv  e-prints  ,

[2]   T .  Wu  and  M .  Tegmark ,  arXiv  e-prints  ,
 arXiv : 2004 . 1 3747   ( 2020) ,  arXiv : 2004 . 1 3747   [stat . ML] .

arXiv : 1 8 1 0 . 1 05 2 5   ( 20 1 8 ) ,  arXiv : 1 8 1 0 . 1 05 2 5
 [9]   S .  Efthymiou ,  J .  Hidary ,  and  S .  Leichenauer ,  arXiv

[physics . comp-ph] .
 e-prints  ,  arXiv : 1 906 . 06329   ( 20 1 9 ) ,  arXiv : 1 906 . 06329

[3]   K .  Kottmann ,  P.  Huembeli ,  M .  Lewenstein ,  and
 [cs . L G] .

A .  Acin ,  arXiv  e-prints  ,  arXiv : 2003 . 09905   ( 2020 ) ,
 [ 1 0]   J .  Wang ,  C .  Roberts ,  G .  Vidal ,  and  S .  Le

arXiv : 2003 . 09905   [quant-ph] .
 ichenauer ,  arXiv  e-prints  ,  arXiv : 2006 . 02 5 1 6   ( 2020) ,

[4]   E .  Miles  Stoudenmire  and  D .  J .  Schwab ,  arXiv  e-prints  ,
 arXiv : 2006 . 02 5 1 6   [cs . L G] .

arXiv : 1 605 . 05 775   ( 20 1 6 ) ,  arXiv : 1 605 . 05 775   [st at . ML] .
 [ 1 1 ]   S .  Cheng ,  L .  Wang ,  T .  Xiang ,  and  P.  Zhang ,  Phys .  Rev .

[5]   I .  G lasser ,  N .  Pancott i ,  and  J .  I .  C irac ,  arXiv  e- print s   ,
 B  9 9 ,   1 5 5 1 3 1   ( 2 0 1 9 ) .

arXiv : 1 806 . 05964   ( 20 1 8 ) ,  arXiv : 1 806 . 05964   [quant-ph] .
 [ 1 2]   J .  Reyes  and  M .  Stoudenmire ,  arXiv  e-prints  ,

[6]   E .  M .  Stoudenmire ,  arXiv  e-prints  ,  arXiv : 1 80 1 . 003 1 5
 arXiv : 200 1 . 08286   ( 2020) ,  arXiv : 200 1 . 08286   [st at . ML] .

( 20 1 7) ,  arXiv : 1 80 1 . 003 1 5   [st at . ML] .
 [ 1 3]   D .  Perez- Garcia,  F .  Verstraete ,  M .  M .  Wolf,  and

[7]   R.  Selvan  and  E .  B .  D am ,  arXiv  e-prints  ,
 J .  I .  C irac ,  arXiv  e-prints  ,  quant-ph/ 0608 1 9 7   ( 2006 ) ,

arXiv : 2004 . 1 0076  ( 2020) ,  arXiv : 2004 . 1 0076   [cs . LG] .
 arXiv : quant-ph/0608 1 97   [quant-ph] .

1 0

[ 1 4]   S .  R.  White ,  Phys .  Rev .  Lett .  69 ,  2863   ( 1 992 ) .
 1 .  S chmidt  decomp osit ion

[ 1 5]   M .  Fannes ,  B .  Nachtergaele ,  and  R.  F .  Werner ,  Commu

nications  in  mathematical  physics  1 44 ,  443   ( 1 992 ) .

¨ Given  an  arbitrary  partition  of  the  N  qubits  into  two

[ 1 6]   S .  Rommer  and  S .  O  stlund ,  Phys .  Rev .  B  5 5 ,  2 1 64

subsets  A  and  B ,  we  can  rewrite  st ate  | Σ i   as

( 1 9 9 7) .

[ 1 7]   G .  Vidal ,  P hys .  Rev .  Lett .  9 1 ,   1 4790 2   ( 2 003 ) ,

- - NΣ

arXiv : quant ph/ 030 1 06 3   [quant ph] . 
  
 ( i ) 
  
 ( i )

# E  E

[ 1 8]   G .  Vidal ,  P hys .  Rev .  Lett .   9 3 ,  040 5 0 2   ( 2 004 ) ,
 | Σ i   =
 X  
x
A
  
x
B
 ,   ( A 2 )

arXiv : quant-ph/03 1 0089   [quant-ph] .
 i= 1

[ 1 9]   Y .  Y .  S hi ,  L .  M .  D uan ,  and  G .  Vidal ,  P hys .  Rev .  A  74 ,

02 2320   ( 2006 ) ,  arXiv : quant-ph/05 1 1 070   [quant-ph] .
 where  we  use  the  fact  that  each  pro duct  st ate
  x
(i)
  can

[2 0]   V .  Murg ,  F .  Verst raet e ,  O .  Legez a ,  and  R .  M .  Noack ,
  ( i ) 
 =  
 ( i ) 
  
 ( i )

b e  expressed  as
  x
  
  
x
A
  
x
B
 .  Alt ernat ively,  we

P hys .  Rev .  B  8 2 ,  2 0 5 1 05   ( 2 0 1 0 ) .
  E  E

[2 1 ]   G .  Vidal ,  Phys .  Rev .  Lett .  1 0 1 ,  1 1 050 1   ( 2008 ) ,
 can  also  rewrite  | Σ i   in  it s  S chmidt  decomp osit ion ,

arXiv: quant-ph/06 1 0099   [quant-ph] .

[2 2]   G .  Evenbly  and  G .  Vidal ,  P hys .  Rev .  B  79 ,   1 44 1 08
 χ

( 2 009 ) ,  arXiv : 0 70 7 . 1 454   [cond- mat . st r- el] . 
 | Σ i   =
 X λ α
  ϕ
Aα
 
   ϕ
αB
 
  ,  ( A3 )

[2 3]   A .  C ichocki ,  arXiv  e-prints  ,  arXiv : 1 40 7 . 3 1 24   ( 20 1 4) ,
 α = 1

arXiv : 1 40 7 . 3 1 2 4   [cs . NA] .

[24]   I .  V .  Oseledets ,  SIAM  Journal  on  Scientific  Computing
 where   ϕ
Aα

 and   ϕ
αB

 form  orthonormal  sets  of  vec

#    	 

33 ,  2 295   ( 20 1 1 ) .
 tors  and  the  S chmidt  rank  χ  is  at  most  NΣ .

[2 5]   H .  W .  Lin ,  M .  Tegmark ,  and  D .  Rolnick ,  Journal  of

St atistical  Physics  1 6 8 ,  1 2 23   ( 20 1 7) ,  arXiv : 1 608 . 082 2 5

[cond-mat . dis-nn] .

[26]   J .  C .  Bridgeman  and  C .  T .  Chubb ,  Journal  of

Physics  A  Mathematical  General  5 0 ,  22300 1  ( 20 1 7) ,

arXiv : 1 603 . 03039   [quant-ph] .

´

[2 7]   R.  Orus ,  Annals  of  P hysics  349 ,   1 1 7   ( 20 1 4) ,

- - FIG .  1 1 .  Schmidt  decomposition  of  | Σ i .

arXiv : 1 306 . 2 1 64   [cond mat . str el] .

[2 8]   F .  Verstraete  and  J .  I .  C irac ,  arXiv  e-prints  ,  cond

mat/0407066  (2004) ,  arXiv: cond-mat/0407066   [cond To   o  from  decom osition   A2  to  decom osition   A3

# - g p ( ) p ( )

mat . st r el] . 
 

and  extract  the  Schmidt  coefficients  { λα
 }  we  w  ill  pro

( i )

ceed  in  two  st eps .  First ,  we  will  map   
x
A
 int o  an

n  E o

intermediate  orthonormal  set    ψ
A
 
  	 of  st ates  on  part

γ

A ,

Appendix  A:  Schmidt  spectrum  and  entanglement

e nt r o py
 
 
 NΣ

A
 =  
 ( i )

ψ
γ
  
 X  
x
A
 ( WA ) iγ ,   ( A4 )

# E

i = 1

# In  this  appendix  we  detail  a  method  for  calculating  the

Schmidt  coefficients  { λα }  of  the  sum  st ate  | Σ` i   in  Eq.
 where  γ  =  1 ,  ·  ·  ·  ,  m  for  some  m  ≤  NΣ ,  by  a  change  of

( 1 0) ,  from  which  we  can  easily  also  extract
  the  en
 tangle basis  given  by  an  NΣ  ×   m  matrix  WA  to  be  determined

ment  entropy  S ( A )  =  −
 P χα
= 1 ( λα ) 
2
 log    ( λα ) 
2
  .  More
 b elow .

generally,  we  consider  N  qubits  in  a  st ate  of  the  form

NΣ

=  
 ( i )

| Σ i   
  
x
 ,   ( A 1 )

# X  E

i = 1

A

F I G .   1 2 .  C onst ruct ion  of  | ψ
 i .

where  t he  NΣ  st ates  {
  x
(i )
  }  are  pro duct  st ates   ( for  in

st ance ,  each  pro duct  st at e
  x
 ( i ) 
  could  b e  t he  result  of
  
 ( i )

i 
 S imilarly,  we  will  map   
x
B
 int o  an  int ermediat e

applying  a  local  feature  map  to  an  N-pixel  image  x
( ) ,
 
 n  E o

as  discussed  in  Sec .  II  A ,  alt hough  t he  sp ecific  origin
 ort honormal  set    ψ
γB
 
  	 ,

of
  x
( i )
  is  not  relevant  here ) .  The  manipulat ions  b e

3 
  
 B
 
 
 T
  
 ( i )

low  carry  a  comput at ional  cost  t hat  scales  as  O ( NΣ ) ,
  ψ
 
  =
 ( WB  ) γi
  
x
B
 ,  ( A 5 )

γ X  E

independently  of  the   (potentially  huge)  dimension  of  the
 i

vector  space  of  the  N  qubits .  Using  this  method  one  can

compute  the  Schmidt  coefficients  for  NΣ  on  the  order  of
 by  a  change  of  basis  given  by  some  m0  ×   NΣ  matrix  WBT 
 ,

T  0

# several  thousands  using  a  laptop .
 where  denotes  matrix  transposition  and  m ≤  NΣ .

1 1

FI G .   1 3 .  Construct ion  of  | ψ
 B i .

# strictly  positive  eigenvalues  of  XA  in  its  diagonal  entries

( ( DA ) γγ  >  0) .  Notice  that  UA  ( and  DA )  can  be  obtained

# from  a  regular  eigenvalue  decomposition  of  XA  by  simply

# ignoring  the  columns  (respectively,  columns  and  rows)

# corresponding  to  vanishing  eigenvalues) .  Finally  we  set

− 1 / 2

WA  ≡  UA  D
A  ,  (A 1 7)

In  terms  of  t hese  ort honormal  set s  of  vectors ,  st ate  | Σ i 
 W − 1
 =  D
 1 / 2
 U
 †
 .  A 1 8

A  A  A ( )

reads

m
 m 0
 †
 = − 1 / 2
 †
 †
 − 1 / 2
 =

Notice  that  W
A XA WA    D
A  U
A UA DA U
A UA D
A

| Σ i   =
 X X Mγγ0
  ψ
γA
 
   ψ
γB0

  ,  ( A6 ) 
 Im ,  and  t hat  t hat  WA WA−  1
 is  a  rank- m  proj ector .

γ= 1
 γ0 = 1
 Similarly,  we  find  the  change  of  basis  matrix  WB  above

# 0  by  building  the  Hermitian ,  positive  semi-definite  NΣ × NΣ

with  M  an  m  ×   m matrix  given  by

matrix  XB  of  scalar  products

= − 1 
 − 1 
 T

M    ( WA  ) ( WB  ) 
 .   ( A 7) 
 i

( XB ) ij  ≡   h x
B(  ) | x
B(j  ) i ,   ( A 1 9 )

− 1 
 − 1 
 -

Here  WA  and  WB  are  (pseudo ) inverses  of  WA  and  WB

# such  that
 by  computing  its  eigenvalue  decomposition

( i ) 
 
 
 
 A 
 
 − 1

x
A
 =
 X  ψ
γ
  ( WA  ) γi ,   ( A 8 ) 
 = †

E XB    UB DB U
B ,  (A20)

γ

( i ) 
 
 
 
 B
   − 1 
 T

x
B
 =
 X  ψ
γ
    ( WB  ) 
  iγ  .   ( A9 ) 
 where  UB  is  an  NΣ  ×   m0
 isometric  matrix  wit h  m0  ≤

E γ
 NΣ  and  DB  is  an  m0  ×   m0  diagonal  matrix  with  strictly

# positive  diagonal  entries ,  and  by  then  setting

# Then ,  from  the  singular  value  decomposition  of  M ,

= †
 − 1 / 2

M    VA SV B ,  (A 1 0) 
 WB  ≡  UB D
B  ,  (A2 1 )

we  obt ain  t he  S chmidt  values  λα  as  t he  singular  values
 †
 − 1 / 2
 †
 †
 − 1 / 2

so  that  W
B XB WB  =  D
B  U
B UB DB U
B UB D
B  =

of  M  (given  by  t he  diagonal  entries  Sα α  of  matrix  S ) 
 − 1
 1 2
 
 − 1

whereas  the  Schmidt  vectors  read
 Im0  .  Notice  that  WB  =  D
B/  U
B† ,  so  that  WB WB  is

a  r ank- m0  proj ect or .

ϕ
Aα

  =
 X  ψ
γA
 
  ( VA ) γα  (A 1 1 ) 
 Ab ove  we  actually  used  the  transp osed  matrices

γ

−

=  
 ( i ) 
 T
 = 1 / 2
 T

X  
x
A
 ( WA VA ) iα  ( A 1 2 ) 
 WB    D
B  U
B  ,  ( A2 2 )

###### E

i
 − 1 
 T  = ∗ 
 1 / 2

( WB  ) 
   U
B  D
B  ,   ( A 2 3 )

ϕ
αB
 
  =
 ( V
B†
 ) α γ
  ψ
γB
 
  ( A 1 3 )

###### X

γ
 
 
 where  ∗  denotes  complex  conj ugation  and  we  used  that

=
 
 V
 †
 WT
  

x
 ( i ) 
 .   A 1 4 
 for  a  unit ary/ isomet ric  mat rix  U  we  have  U
 †  ≡  U
 ∗ T  =

X ( B B  ) α i
  B
 E ( ) − 1  − 1
 T  = ∗

i
 U
 and  t herefore   ( U
 ) 
   U
 .

2 .  Matrices  WA  and  WB

# In  order  to  find  matrix  WA  above  we  first  build  the

Hermitian,  positive  semi-definite  NΣ  ×   NΣ  matrix  XA  of

# scalar  products

( XA ) ij  ≡   h x
A( i ) 
 | x
A(j  ) 
 i .   ( A 1 5 )

# We  then  compute  its  eigenvalue  decomposition

FIG .  1 4 .  An  equivalent  expression  for  | Σ i .

# X =  U D U
 †
   A 1 6 
 Finally,  collecting  all  these  terms  together  we  can  ex

A  A A A , ( )

# press  the  matrix  M  in  Eq.   (A7)  as

where  UA  is  an  NΣ × m  isometric  matrix   (that  is ,  U
A†
 UA  =

= − 1 
 − 1 
 T  = 1 / 2
 †
 ∗ 
 1 / 2

Im )  and  DA  is  an  m  ×   m  diagonal  matrix  with  the  m
 M    ( WA  ) ( WB  ) 
   D
A  U
A  U
B  D
B  ,  (A24)

1 2

whereas  the  Schmidt  bases  read
 and  XB ,  and  build  and  singular  value  decompose  matrix

M  with  a  cost  at  most  O ( NΣ3
 ) .

A
 =  
 ( i )

ϕ
α
  
 X  
x
A
 ( WA VA ) i α  ( A 2 5 )

E

i

( i ) 
 − 1 / 2

=
 X  
x
A
 ( UA  D
A  VA ) iα ,  ( A2 6 )

E

i

B
 = † 
 T
  
 ( i )

ϕ
α
  
 X ( V
B WB  ) α i
  
x
B
 ( A 2 7)

E

i

= † 
 − 1 / 2
 T
  
 ( i )

X ( V
B  D
B  U
B  ) αi
  
x
B
 .   ( A 2 8 ) 
 FI G .   1 5 .  Anot her  equivalent  expression  for  | Σ i .  This  expres

E

i
 sion  elucidates  how  we  can  obtain  the  Schmidt  coefficients

λα .

Importantly,  we  can  build  and  diagonalize  matrices  XA
