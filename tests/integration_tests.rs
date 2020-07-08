use clblas_sys::{clblasStatus, clblasSetup, clblasSaxpy};
use ocl::ProQue;

#[test]
fn test_saxpy() {
    let n = 1024;
    
    let pro_que = ProQue::builder()
        .src("")
        .dims(n)
        .build()
        .unwrap();
        
    let status = unsafe { clblasSetup() };
    assert_eq!(status, clblasStatus::clblasSuccess);

    let x_buf = pro_que.create_buffer::<f32>().unwrap();
    let y_buf = pro_que.create_buffer::<f32>().unwrap();
    
    let alpha: f32 = 1.0;
    let offx = 0;
    let incx = 1;
    let offy = 0;
    let incy = 1;
    
    let num_queues = 1;
    let num_events_in_waitlist = 0;
    
    let status = unsafe { 
        let mut queues = [pro_que.queue().as_ptr()];
        let events_waitlist = std::ptr::null();
        let events = std::ptr::null_mut();
        clblasSaxpy( 
            n,
            alpha,
            x_buf.as_ptr() as clblas_sys::cl_mem,
            offx,
            incx,
            y_buf.as_ptr() as clblas_sys::cl_mem,
            offy,
            incy,
            num_queues,
            queues.as_mut_ptr() as *mut clblas_sys::cl_command_queue,
            num_events_in_waitlist,
            events_waitlist,
            events
        )
    };
    
    assert_eq!(status, clblasStatus::clblasSuccess);
    
    let mut y_vec = vec![0.0f32; n];
    y_buf.read(&mut y_vec).enq().unwrap();
}
