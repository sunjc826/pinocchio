According to PGHR13, Pinocchio has linear prover overhead. Note that the arithmetic circuit evaluates both branches of a conditional, thus, if we have the following, we need to do the expensive computation in any case. 
```c
if (cond)
{
    // do absolutely nothing
}
else
{
    // do expension computation
}
```
Hence, the more accurate of stating the asymptotics may be: Pinocchio has linear prover overhead $O(T)$, where $T$ is the compile time *upper bound* on the size of the computation. Thus, $T$ will take into account the most expensive branch, or equivalently, add the size of all branches together.


According to BCGTV13 (SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge), traditional circuit generators (such as Pinocchio) have in the worst case quadratic overhead $O(T^2)$, where $T$ is the number of steps run. To justify this, BCGT13 raises an example of selecting 1 vector from $n$ vectors, each vector of bit-size $l$. In naive/traditional multiplexing generators, the overhead is claimed to be $\Theta(n(log l + n))$ gates, as compared to BCGTV13's $O(nl / \mathbb{F})$. And by setting $n = l$, the traditional overhead is $\Theta(n^2)$. If we only look at the selection step, i.e. then perhaps $T = l$ (i.e. $l$ assignments.)
```
vec_1, vec_2, ... vec_n
selection = vec[k];
```
But if we take into account the initialization of the $n$ vectors, then in my opinion, we would get $T = n * l$ again, so maybe the seeming contradiction between PGHR and BCGTV is due to slight differences in how they argue what $T$ is.
i.e. this seems similar to the difference between total space and auxiliary space, for e.g. an inplace sorting algorithm use O(1) auxiliary space.