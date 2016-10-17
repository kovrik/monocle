import java.io.Serializable;

public class Test implements Serializable, Runnable {

  private static final long serialVersionUID = 1L;

  private static final int intv = 38944985;
  private static final int int_max = Integer.MAX_VALUE;
  private static final int int_min = Integer.MIN_VALUE;

  private static final long l = 1234;
  private static final long min = Long.MAX_VALUE;
  private static final long max = Long.MIN_VALUE;

  private static final double d = 3452.776655d;
  private static final double nand = Double.NaN;
  private static final double dmax = Double.MAX_VALUE;
  private static final double dmin = Double.MIN_VALUE;

  private static final double dinfp = Double.POSITIVE_INFINITY;
  private static final double dinfm = Double.NEGATIVE_INFINITY;

  private static final float f1 = -3345.345f;
  private static final float nanf = Float.NaN;
  private static final float fmax = Float.MAX_VALUE;
  private static final float fmin = Float.MIN_VALUE;

  private static final float finfp = Float.POSITIVE_INFINITY;
  private static final float finfm = Float.NEGATIVE_INFINITY;

  public static void main(String[] args) {
    String s = "Hello";
    System.out.println(s);
  }

  public void run() {
  }

}
