package io.github.imurx.arboard;

import java.lang.ref.Cleaner;

public class ImageData implements AutoCloseable {
    private static native long imageDataNew(int width, int height, byte[] image);
    private static native byte[] imageDataGetBytes(long ptr);
    private static native void imageDataDrop(long ptr);
    
    private final Cleaner.Cleanable cleanable;
    protected final long ptr;

    public ImageData(int width, int height, byte[] image) {
        this(imageDataNew(width, height, image));
    }
    
    protected ImageData(long ptr) {
        this.ptr = ptr;
        cleanable = Clipboard.cleaner.register(this, () -> {
            imageDataDrop(this.ptr);
        });
    }

    public byte[] getImage() {
        return imageDataGetBytes(this.ptr);
    }

    @Override
    public void close() {
        this.cleanable.clean();
    }
}
