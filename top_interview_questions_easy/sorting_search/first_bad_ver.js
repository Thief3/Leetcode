/******************************************************************************

First Bad Version

You are a product manager and currently leading a team to develop a new product.
Unfortunately, the latest version of your product fails the quality check. Since
each version is developed based on the previous version, all the versions after
a bad version are also bad.

Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad
one, which causes all the following ones to be bad.

You are given an API bool isBadVersion(version) which will return whether version
is bad. Implement a function to find the first bad version. You should minimize
the number of calls to the API.

Example:

Given n = 5, and version = 4 is the first bad version.

call isBadVersion(3) -> false
call isBadVersion(5) -> true
call isBadVersion(4) -> true

Then 4 is the first bad version. 

******************************************************************************/

/**
 * Definition for isBadVersion()
 * 
 * @param {integer} version number
 * @return {boolean} whether the version is bad
 * isBadVersion = function(version) {
 *     ...
 * };
 */

/**
 * @param {function} isBadVersion()
 * @return {function}
 */
var solution = function(isBadVersion) {
    /**
     * @param {integer} n Total versions
     * @return {integer} The first bad version
     */
    return function(n) {
        var lower = 0;
        var upper = n;
        var iter = Math.floor((lower + upper)/2);
        while(true){
            var current = isBadVersion(iter);
            if(isBadVersion(iter - 1) == false && current){
                break;
            }
            else if(current == false && isBadVersion(iter + 1)){
                iter = iter + 1;
                break
            }
            else if(current){
                upper = iter;
                iter = Math.floor((lower + upper)/2);
            }
            else{
                lower = iter;
                iter = Math.floor((lower + upper)/2)
            }
        }
        return iter;
    };
};
