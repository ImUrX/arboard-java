package io.github.imurx.arboard;
import java.io.IOException;
import java.lang.ref.Cleaner;

import com.nativeutils.NativeUtils;

public class Clipboard implements AutoCloseable {
    protected static final Cleaner cleaner = Cleaner.create();

    private static native long clipboardNew();
    private static native String clipboardGetText(long clipboard_ptr);
    private static native void clipboardSetText(long clipboard_ptr, String input);
    private static native long clipboardGetImage(long clipboard_ptr);
    private static native void clipboardSetImage(long clipboard_ptr, long image_ptr);
    private static native void clipboardDrop(long clipboard_ptr);

    static {
        try {
            NativeUtils.loadLibraryFromJar("/natives/" + System.mapLibraryName("arboard_java"));
        } catch (IOException e1) {
            throw new RuntimeException(e1);
        }
    }

    private final Cleaner.Cleanable cleanable;
    private final long ptr;

    public Clipboard() {
        this.ptr = clipboardNew();
        cleanable = cleaner.register(this, () -> {
            clipboardDrop(this.ptr);
        });
    }

    public String getText() {
        return clipboardGetText(this.ptr);
    }

    public void setText(String input) {
        clipboardSetText(this.ptr, input);
    }

    public ImageData getImage() {
        return new ImageData(clipboardGetImage(this.ptr));
    }

    public void setImage(ImageData image) {
        clipboardSetImage(this.ptr, image.ptr);
    }

    @Override
    public void close() {
        cleanable.clean();
    }
}
