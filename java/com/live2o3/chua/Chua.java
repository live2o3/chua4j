package com.live2o3.chua;

public class Chua {
    public static final int OK = 0;
    public static final int NULL = 1;
    public static final int INVALID = 2;
    public static final int UPLOAD = 3;

    public static native int upload(String url, String path, int chunkSize, int parallel);

    static {
        System.loadLibrary("chua4j");
    }
}
