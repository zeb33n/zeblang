declare void @exit(i32)
define i32 @main() {
entry:
    %1 = alloca [3 x i32], align 4
    %2 = getelementptr inbounds [3 x i32], [3 x i32]* %1, i32 0, i32 0
    store i32 1, i32* %2, align 4
    %3 = getelementptr inbounds [3 x i32], [3 x i32]* %1, i32 0, i32 1
    store i32 2, i32* %3, align 4
    %4 = getelementptr inbounds [3 x i32], [3 x i32]* %1, i32 0, i32 2
    store i32 3, i32* %4, align 4
    %5 = alloca i32, align 4
    store ptr %1, i32* %5, align 4
    %6 = getelementptr inbounds [3 x i32], [3 x i32]* %5 , i32 0, i32 2
    %7 = load i32, i32* %6, align 4
    call void @exit(i32 %7)
    ret i32 1
}
