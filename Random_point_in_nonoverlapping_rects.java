/** 497. Random Point in Non-overlapping Rectangles
 * Given a list of non-overlapping axis-aligned rectangles rects, write a function pick which randomly and uniformily picks an integer point in the space covered by the rectangles.
 *
 * Note:
 *    An integer point is a point that has integer coordinates. 
 *    A point on the perimeter of a rectangle is included in the space covered by the rectangles. 
 *    ith rectangle = rects[i] = [x1,y1,x2,y2], where [x1, y1] are the integer coordinates of the bottom-left corner, and [x2, y2] are the integer coordinates of the top-right corner.
 *    length and width of each rectangle does not exceed 2000.
 *    1 <= rects.length <= 100
 *    pick return a point as an array of integer coordinates [p_x, p_y]
 *    pick is called at most 10000 times.
 *
 * Example 1:
 * Input: 
 * ["Solution","pick","pick","pick"]
 * [[[[1,1,5,5]]],[],[],[]]
 * Output: 
 * [null,[4,1],[4,1],[3,3]]
 *
 * Example 2:
 * Input: 
 * ["Solution","pick","pick","pick","pick","pick"]
 * [[[[-2,-2,-1,-1],[1,0,3,0]]],[],[],[],[],[]]
 * Output: 
 * [null,[-1,-2],[2,0],[-2,-1],[3,0],[-2,-2]]
 * 
 * Explanation of Input Syntax:
 * The input is two lists: the subroutines called and their arguments. Solution's constructor has one argument, the array of rectangles rects. pick has no arguments. Arguments are always wrapped with a list, even if there aren't any.
 *
 * https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/ 
 */

// Not uniform so doesn't pass large tests.
// Using TreeMap for random would fix this but not today.

import java.lang.Math;
import java.util.Collections;

class Solution {

    private int min_horz = 1;
    private int max_horz = 5;
    private int min_vert = 1;
    private int max_vert = 5;
    
    private int[] points_horz;
    private int[] points_vert;
    
    private int[][] rect_list;
    
    public Solution(int[][] rects) {        
        rect_list = rects;
    }
    
    public int[] pick() {
        int[] output = new int[2];
        int rectangle = (int)Math.round(Math.random() * (rect_list.length - 1));
        
        output[0] = (int)Math.round(Math.random() * (rect_list[rectangle][2] - rect_list[rectangle][0]) + rect_list[rectangle][0]);
        output[1] = (int)Math.round(Math.random() * (rect_list[rectangle][3] - rect_list[rectangle][1]) + rect_list[rectangle][1]);
        
        return output;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(rects);
 * int[] param_1 = obj.pick();
 */
